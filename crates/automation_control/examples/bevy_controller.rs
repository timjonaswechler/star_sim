//! Controller smoke test for the rendered Controlled Session.
use automation_control::{
    Command, Handle,
    driver::{LaunchSpec, LaunchTargetKind, Session, SessionOptions},
    observation::{Projection, Request as ObservationRequest, Selector},
    pointer::{Button, Command as PointerCommand},
};
use serde_json::Value;
use std::{
    f32::consts::TAU,
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Options {
    duration: Duration,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Bounds {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = parse_options(std::env::args().skip(1))?;
    let launch = LaunchSpec {
        package: "bevy_example".into(),
        kind: LaunchTargetKind::Binary,
        target: "bevy_example".into(),
        features: vec!["automation".into()],
        arguments: vec![],
    };
    let mut session = Session::spawn(&launch, SessionOptions::new(Duration::from_secs(180)))?;
    let ready = session.ready()?;
    assert_eq!(ready.version, 2);
    assert_eq!(ready.mode, automation_control::RunMode::Rendered);

    let targets = observe(&mut session, Selector::Targets)?;
    let button = find_named(&targets, "button")?;
    let background = find_named(&targets, "background")?;
    let background_handle: Handle = serde_json::from_value(background["entity"].clone())?;
    let background_bounds = Bounds::from_value(&background["bounds"])?;
    let button_center = center(&button["bounds"])?;

    pointer(
        &mut session,
        PointerCommand::Move {
            surface: None,
            position: button_center,
        },
    )?;
    pointer(
        &mut session,
        PointerCommand::Press {
            button: Button::Primary,
        },
    )?;
    pointer(
        &mut session,
        PointerCommand::Release {
            button: Button::Primary,
        },
    )?;

    let menu = observe(&mut session, Selector::Targets)?;
    let item = find_named(&menu, "item-fuchsia")?;
    let item_center = center(&item["bounds"])?;
    pointer(
        &mut session,
        PointerCommand::Move {
            surface: None,
            position: item_center,
        },
    )?;
    pointer(
        &mut session,
        PointerCommand::Press {
            button: Button::Primary,
        },
    )?;
    pointer(
        &mut session,
        PointerCommand::Release {
            button: Button::Primary,
        },
    )?;

    let state = session.request(Command::Observe(ObservationRequest::new(
        Selector::Entity(background_handle),
        Projection::Components {
            type_paths: vec!["bevy_example::SessionState".into()],
        },
    )))?;
    let value =
        &state.result.as_ref().unwrap()["items"][0]["components"]["bevy_example::SessionState"];
    assert_eq!(value["status"], "available");
    assert_eq!(value["value"]["menu_open"], false);
    assert_eq!(value["value"]["selected_item"], "fuchsia");

    move_pointer_in_circles(&mut session, background_bounds, options.duration)?;
    session.shutdown()?;
    Ok(())
}

fn parse_options<I, S>(args: I) -> Result<Options, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args = args.into_iter();
    let mut duration = None;
    while let Some(argument) = args.next() {
        match argument.as_ref() {
            "--circle-seconds" => {
                if duration.is_some() {
                    return Err("--circle-seconds may only be supplied once".into());
                }
                let seconds = args
                    .next()
                    .ok_or("--circle-seconds requires a positive integer")?
                    .as_ref()
                    .parse::<u64>()
                    .map_err(|_| "--circle-seconds requires a positive integer")?;
                if seconds == 0 {
                    return Err("--circle-seconds requires a positive integer".into());
                }
                duration = Some(Duration::from_secs(seconds));
            }
            unknown => return Err(format!("unknown argument {unknown:?}")),
        }
    }
    Ok(Options {
        duration: duration.unwrap_or_default(),
    })
}

impl Bounds {
    fn from_value(value: &Value) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            x: value["x"].as_f64().ok_or("bounds.x missing")? as f32,
            y: value["y"].as_f64().ok_or("bounds.y missing")? as f32,
            width: value["width"].as_f64().ok_or("bounds.width missing")? as f32,
            height: value["height"].as_f64().ok_or("bounds.height missing")? as f32,
        })
    }
}

fn position_on_circle(bounds: Bounds, elapsed: Duration) -> [f32; 2] {
    let center = [
        bounds.x + bounds.width / 2.0,
        bounds.y + bounds.height / 2.0,
    ];
    let radius = bounds.width.min(bounds.height) * 0.3;
    let angle = elapsed.as_secs_f32() * TAU;
    [
        center[0] + radius * angle.cos(),
        center[1] + radius * angle.sin(),
    ]
}

fn move_pointer_in_circles(
    session: &mut Session,
    bounds: Bounds,
    duration: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    if duration.is_zero() {
        return Ok(());
    }
    eprintln!(
        "moving the virtual pointer in circles for {} seconds",
        duration.as_secs()
    );
    let started = Instant::now();
    while started.elapsed() < duration {
        pointer(
            session,
            PointerCommand::Move {
                surface: None,
                position: position_on_circle(bounds, started.elapsed()),
            },
        )?;
        thread::sleep(Duration::from_millis(16));
    }
    eprintln!("virtual pointer circle complete");
    Ok(())
}

fn observe(
    session: &mut Session,
    selector: Selector,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let response = session.request(Command::Observe(ObservationRequest::new(
        selector,
        Projection::Summary,
    )))?;
    Ok(response
        .result
        .and_then(|value| value["items"].as_array().cloned())
        .ok_or("observation did not return items")?)
}

fn pointer(
    session: &mut Session,
    command: PointerCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    session.request(Command::Pointer(command))?;
    Ok(())
}

fn find_named<'a>(items: &'a [Value], name: &str) -> Result<&'a Value, Box<dyn std::error::Error>> {
    items
        .iter()
        .find(|item| item["name"] == name)
        .ok_or_else(|| format!("target {name:?} not found in {items:?}").into())
}

fn center(bounds: &Value) -> Result<[f32; 2], Box<dyn std::error::Error>> {
    let x = bounds["x"].as_f64().ok_or("bounds.x missing")? as f32;
    let y = bounds["y"].as_f64().ok_or("bounds.y missing")? as f32;
    let width = bounds["width"].as_f64().ok_or("bounds.width missing")? as f32;
    let height = bounds["height"].as_f64().ok_or("bounds.height missing")? as f32;
    Ok([x + width / 2.0, y + height / 2.0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_seconds_enables_timed_pointer_motion() {
        let options = parse_options(["--circle-seconds", "60"]).unwrap();
        assert_eq!(options.duration, Duration::from_secs(60));
    }

    #[test]
    fn pointer_completes_one_circle_per_second_inside_the_bounds() {
        let bounds = Bounds {
            x: 10.0,
            y: 20.0,
            width: 200.0,
            height: 100.0,
        };
        assert_eq!(position_on_circle(bounds, Duration::ZERO), [140.0, 70.0]);
        let quarter = position_on_circle(bounds, Duration::from_millis(250));
        assert!((quarter[0] - 110.0).abs() < 0.001);
        assert!((quarter[1] - 100.0).abs() < 0.001);
    }
}
