use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum PngError {
    Read {
        path: PathBuf,
        error: std::io::Error,
    },
    Invalid {
        path: PathBuf,
    },
    UnexpectedSize {
        path: PathBuf,
        width: u32,
        height: u32,
        expected: [u32; 2],
    },
}

impl fmt::Display for PngError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, error } => {
                write!(formatter, "failed to read {}: {error}", path.display())
            }
            Self::Invalid { path } => write!(formatter, "{} is not a valid PNG", path.display()),
            Self::UnexpectedSize {
                path,
                width,
                height,
                expected,
            } => write!(
                formatter,
                "unexpected PNG size for {}: {width}x{height}, expected {}x{}",
                path.display(),
                expected[0],
                expected[1]
            ),
        }
    }
}

impl std::error::Error for PngError {}

pub fn validate_png(path: impl AsRef<Path>, expected: [u32; 2]) -> Result<(), PngError> {
    let path = path.as_ref();
    let data = fs::read(path).map_err(|error| PngError::Read {
        path: path.to_path_buf(),
        error,
    })?;
    if data.len() < 24 || &data[..8] != b"\x89PNG\r\n\x1a\n" || &data[12..16] != b"IHDR" {
        return Err(PngError::Invalid {
            path: path.to_path_buf(),
        });
    }
    let width = u32::from_be_bytes(data[16..20].try_into().expect("PNG width has four bytes"));
    let height = u32::from_be_bytes(data[20..24].try_into().expect("PNG height has four bytes"));
    if [width, height] != expected {
        return Err(PngError::UnexpectedSize {
            path: path.to_path_buf(),
            width,
            height,
            expected,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn validates_png_signature_and_dimensions() {
        let path =
            std::env::temp_dir().join(format!("automation-control-png-{}.png", std::process::id()));
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0\x02\0\0\0\x01")
            .unwrap();
        assert!(validate_png(&path, [2, 1]).is_ok());
        assert!(matches!(
            validate_png(&path, [1, 1]),
            Err(PngError::UnexpectedSize { .. })
        ));
        fs::remove_file(path).ok();
    }
}
