# Automation Control auf Reflection und Virtual Pointer neu aufbauen

Implementiere den ersten vertikalen Schnitt der Neustrukturierung aus Issue #49. Der Schnitt muss beweisen, dass ein Host die Dummy-App als isolierte Controlled Session startet, ihren aktuellen Bevy-Zustand gezielt beobachtet und einen virtuellen Pointer durch Bevys normale Picking-Pipeline steuert.

## Vor dem Ändern

1. Lies `AGENTS.md`, den Abschnitt "Controlled play and automation" in `CONTEXT.md`, Issue #49 und `docs/adr/0003-isolate-automation-in-controlled-sessions.md`.
2. Prüfe `git status` und den vollständigen Diff. Der Arbeitsbaum enthält einen begonnenen Umbau mit absichtlichen Löschungen. Erhalte diese Arbeit. Setze keine Dateien zurück und stelle gelöschte Legacy-Module nicht ungefragt wieder her.
3. Untersuche den aktuellen Stand von:
   - `crates/automation_control/src/`
   - `crates/automation_control/bevy_example/`
   - `crates/automation_control/examples/bevy_curosr.rs`
   - `crates/automation_control/src/driver/`
4. Prüfe die Bevy-0.19.1-Quellen für `PointerInput`, `PointerInputSettings`, `PointerId`, `PointerLocation`, `PointerPress`, `PointerInteraction`, `AppTypeRegistry` und `ReflectComponent`.

Dieser Schritt ist abgeschlossen, wenn du den vorhandenen Transport und Driver von den zu ersetzenden Target-, Observation-, Kamera- und Action-Abstraktionen getrennt hast und jede bestehende Änderung im Arbeitsbaum zuordnen kannst.

## Festgelegte Architektur

### Prozessmodell

Der Host startet `bevy_example` als eigenen Child-Prozess mit dem Feature `automation`. Der Child liest JSONL über sein gepiptes stdin und schreibt ausschließlich Protokollnachrichten nach stdout. Logs gehen nach stderr. Ohne das Feature läuft `bevy_example` als normal bedienbare Bevy-App.

Die normale Star-Sim-App bleibt ohne Automation baubar. Der erste Schnitt verändert ihre fachliche UI nicht.

### Steuerung und Beobachtung

Automation hat zwei getrennte Interfaces:

```text
Host
  -> Pointer-Kommandos
  -> Bevy PointerInput
  -> Picking, Hover und Pointer-Events
  -> Anwendung

Host
  -> Observe-Anfrage
  -> read-only World- und Reflection-Abfrage
  -> begrenzte JSON-Antwort
```

Pointer-Kommandos beschreiben Eingabeübergänge. Sie behaupten nicht, welche UI-Interaktion daraus entsteht. Bevy und die Anwendung erzeugen `Over`, `Out`, `Press`, `Release`, `Click` und Drag-Verhalten. Der Controller beobachtet danach den Zustand und entscheidet über den nächsten Schritt.

Reflection ist read-only. Das Protokoll darf keine beliebigen Component-Werte verändern. Zustandsänderungen erfolgen im ersten Schnitt ausschließlich über den virtuellen Pointer und Shutdown.

### Capabilities

Eine Capability beschreibt ein installiertes Kontroll- oder Beobachtungsmodul, nicht den aktuellen World-Inhalt. Ein vorhandener UI-Button ist keine Capability.

Entferne den vom Aufrufer übergebenen `Vec<String>` aus `AutomationControlPlugin::rendered_stdio` und `with_io`. `AutomationControlPlugin::rendered_stdio()` erhält keine Capability-Liste; `with_io` erhält den `RunMode` explizit. Installierte Module registrieren ihre stabile Unterstützung selbst. `ready` darf zum Beispiel `pointer` und verfügbare Observation-Scopes melden. Die Liste wird aus der App-Komposition abgeleitet.

Das Fehlen eines Fensters, Pointers oder Targets ist dynamischer Zustand. Ein betroffener Request liefert eine typisierte Fehlermeldung oder eine leere Observation. Er verändert nicht die ausgehandelten Capabilities.

### Entity-Identität

`AutomationTarget` wird ein kleiner Marker ohne String-ID, Rolle oder Action-Liste. Bevy-Entities sind die Handles für die laufende Session. Ein Entity-Handle muss Index und Generation beziehungsweise eine verlustfreie Darstellung von `Entity::to_bits()` enthalten. Vermeide eine nackte JSON-Zahl für 64-Bit-Werte, die JavaScript nicht verlustfrei darstellen kann.

Entity-Handles sind nur innerhalb einer Session gültig. Recordings oder frische Sessions müssen Entities erneut beobachten und auswählen. Implementiere in diesem Schnitt keine neue dauerhafte semantische ID-Schicht.

## Protokoll des ersten Schnitts

Die Session meldet ihre Protokollversion einmal in `ready`. Einzelne Requests wiederholen sie nicht. Der Driver vergibt pro Session eine fortlaufende Sequenz und ordnet damit Requests und Responses zu; REPL, Skript oder Agent liefern weder Protokollversion noch Request-ID. Halte eine optionale spätere Korrelations-ID möglich, ohne sie im ersten synchronen Driver-Interface zu verlangen.

Die Wire-Form folgt diesem Modell:

```text
ready:    version, mode, controls, observation_scopes
request:  sequence, command
response: sequence, status, result | error
```

Ersetze die derzeit breite Command-Aufzählung durch eine kleine, gruppierte Schnittstelle. Die genauen Rust-Namen dürfen sich an die bestehende Struktur anpassen, die folgenden Fähigkeiten und Grenzen sind verbindlich.

```rust
enum Command {
    Observe(ObservationRequest),
    Pointer(PointerCommand),
    Shutdown,
}

enum PointerCommand {
    Move { surface: Option<EntityHandle>, position: [f32; 2] },
    Press { button: PointerButton },
    Release { button: PointerButton },
    Scroll { delta: [f32; 2] },
}
```

`position` verwendet logische Pixel der gewählten Render-Fläche. Ohne `surface` wird das eindeutige primäre Fenster verwendet. Lehne mehrdeutige oder fehlende Flächen typisiert ab. Speichere Pointerposition und gedrückte Pointertasten pro Controlled Session.

`PointerButton` bezeichnet die Taste des Zeigegeräts, nicht einen UI-Button. Unterstütze mindestens `primary`, `secondary` und `middle` im Datentyp. Der End-to-End-Test muss nur `primary` verwenden.

Es gibt im ersten Schnitt kein wire-level `click` und keine Action-Liste an Targets. Ein späteres `click` kann als Host-Makro aus Move, Press und Release entstehen. Damit bleibt die Anforderung aus Issue #49 erreichbar, ohne die alte semantische Target-Registry wieder einzuführen.

### Observation

Verwende eine gemeinsame `Observe`-Anfrage statt `InspectUi`, `InspectScene`, `InspectSelection` und `InspectCamera`.

Der erste Schnitt unterstützt mindestens diese Selektoren:

```rust
enum ObservationSelector {
    Targets,
    Ui,
    Pointers,
    Entity(EntityHandle),
}
```

Und mindestens diese Projektionen:

```rust
enum ObservationProjection {
    Summary,
    ComponentNames,
    Components { type_paths: Vec<String> },
    Hierarchy { depth: u8 },
}
```

Jede mengenwertige Anfrage hat ein hart validiertes `limit`. Führe einen Cursor ein, wenn eine Antwort das Limit überschreiten kann. Sortiere Ergebnisse deterministisch nach Entity-Handle.

#### Erwartete Inhalte

- `Targets` wählt Entities mit `AutomationTarget`.
- `Ui` wählt Bevy-UI-Entities und liefert in `Summary` mindestens Entity-Handle, optionalen `Name`, Parent, Children, Sichtbarkeit und berechnete globale Bounds, soweit Bevy sie bereitstellt.
- `Pointers` liefert Pointer-Entity, ID, aktuelle Position, gedrückte Tasten und aktuelle Interaction beziehungsweise Hover-Bezüge.
- `Entity` prüft, dass das Handle noch lebt und zur aktuellen Generation gehört.
- `ComponentNames` enthält alle vorhandenen Component-Type-Paths, auch wenn deren Wert nicht reflektiert werden kann.
- `Components` liefert ausschließlich die angeforderten Werte. Nicht registrierte oder nicht darstellbare Werte erhalten pro Component einen expliziten Status statt den gesamten Request scheitern zu lassen.
- `Hierarchy` begrenzt und validiert die Tiefe. Zyklen oder despawnte Beziehungen dürfen die Anfrage nicht blockieren.

Benutze `AppTypeRegistry`, `ReflectComponent` und die World-Metadaten direkt. Implementiere keinen zweiten dauerhaft synchronisierten Observation-Cache. Berechne eine Observation auf Anfrage aus dem aktuellen World. Ergänze kleine kontextuelle Zusammenfassungen dort, wo rohe Reflection dem Controller keine brauchbare Information gibt, insbesondere bei UI-Bounds und Pointerzustand.

## Bevy-Pointer

Die Controlled Session speist direkt `bevy_picking::pointer::PointerInput` ein. Sie injiziert keine Betriebssystemereignisse und bewegt keinen OS-Cursor.

Nutze `PointerId::Mouse` für den virtuellen Hauptpointer, wenn dies für Bevys `Hovered`- und `DirectlyHovered`-Komponenten erforderlich ist. Deaktiviere in der Controlled Session die nativen Maus- und Touch-Quellen von `PointerInputPlugin` über `PointerInputSettings`. Beachte zusätzlich ADR 0003 und die dort festgelegte Entfernung von `InputPlugin`; registriere nur die Nachrichten und leeren Zustandsressourcen, welche die verbleibenden Bevy-Systeme tatsächlich benötigen.

Ordne die Systeme explizit:

1. JSONL-Requests empfangen.
2. Pointer-Kommandos in `PointerInput` übersetzen.
3. Bevys `PickingSystems::ProcessInput`, Backend und Hover ausführen lassen.
4. Request erst in `Last` abschließen, nachdem die Anwendung die daraus entstandenen Events verarbeiten konnte.

Nacheinander gesendete und bestätigte Move-, Press- und Release-Requests müssen in getrennten App-Updates verarbeitet werden. Ein einzelner Request darf nicht intern einen vollständigen Klick vortäuschen.

## Dummy-App und Host

### `bevy_example`

Behalte das Context-Menu-Beispiel als Anwendung unter Test. Markiere mindestens:

- den Hintergrund oder die primäre Pointerfläche,
- den Button, der das Context Menu öffnet,
- dynamisch erzeugte Context-Menu-Einträge.

Der Marker enthält keine Action-Metadaten. Stelle beobachtbaren Anwendungszustand bereit, mit dem der Test beweisen kann, dass ein virtueller Pointer den echten Observer-Pfad ausgelöst hat, etwa geöffnetes Menü und aktuelle Hintergrundfarbe.

Die Feature-Komposition ist:

```text
keine Features: normale native Bevy-App
automation: Controlled Session plus JSONL, Reflection-Observation und Virtual Pointer
```

Ein visueller Inspector und `bevy-inspector-egui` gehören nicht zu dieser Komposition.

### Host-Beispiel

Benenne `examples/bevy_curosr.rs` korrekt und eindeutig um. Das Beispiel ist ein Controller und startet `bevy_example` über den vorhandenen `driver::Session` und einen `LaunchSpec` mit dem Feature `automation`.

Der scripted Smoke-Pfad führt aus:

1. `ready` abwarten.
2. UI oder Targets beobachten und den Context-Menu-Button finden.
3. Pointer zu dessen Bounds bewegen.
4. Move-Antwort abwarten.
5. Primary press senden und Antwort abwarten.
6. Primary release senden und Antwort abwarten.
7. Beobachten, dass das Context Menu existiert.
8. Einen dynamischen Menüeintrag beobachten und auf dieselbe Weise betätigen.
9. Beobachten, dass sich der Anwendungszustand geändert hat.
10. Session geordnet herunterfahren.

Ein kleiner manueller Kommandoloop darf denselben Driver verwenden, sobald der scripted Pfad grün ist. Halte ihn dünn und lokal im Beispiel. Die öffentliche REPL von `star_sim_debug`, Tastatur, Text, Recording, Replay, Screenshots, Kameraoperationen und mehrere Instanzen sind Folgearbeit.

## Bestehenden Code abbauen

Erhalte und nutze, soweit sie zum neuen Interface passen:

- `transport.rs` und die Trennung stdout/stderr,
- `driver::Session` und Child-Prozess-Lifecycle,
- `driver::LaunchSpec`,
- Diagnose- und Report-Code, sofern er ohne die alte Command-Aufzählung weiter kompiliert.

Entferne oder ersetze im ersten Schnitt:

- caller-gelieferte Capability-Strings,
- `AutomationTarget`-IDs, Rollen und Actions,
- `TargetRegistry` und den synchronisierten Observation-Cache,
- spezielle Inspect-Kommandos,
- den eingebauten semantischen Click-Pfad,
- verwaiste Kamera-, Koordinaten-, Artifact- und Wait-Typen, die auf bereits gelöschte Module zeigen,
- veraltete Beispiele, Tests und README-Abschnitte.

Erhalte keine Abstraktion nur, um alte interne Tests unverändert zu lassen. Schreibe Tests gegen das neue Protokoll und die neue öffentliche Schnittstelle. Halte zusammengehörige Pointer-, Observation- und Driver-Symbole gemäß den Naming Rules aus `AGENTS.md` in gemeinsamen Modulen.

## Tests

Schreibe fokussierte Tests für mindestens:

1. Protokoll-Decoding und Validierung aller Pointervarianten.
2. Verlustfreie Entity-Handle-Konvertierung und Ablehnung despawnter oder falscher Generationen.
3. `Targets`, `Ui`, `Pointers`, `Entity`, Limits und Hierarchietiefe.
4. Component-Namen für nicht reflektierbare Components.
5. Ausgewählte reflectierbare Component-Werte und explizite Nicht-Verfügbarkeit.
6. Native Pointer-Nachrichten erzeugen keine Context-Menu-Aktion in der Controlled Session.
7. Virtuelle Move-, Press- und Release-Nachrichten durchlaufen Bevys Picking-Pipeline und lösen die vorhandenen Pointer-Observer aus.
8. stdout der Controlled Session enthält nur JSONL.
9. Der Driver startet `bevy_example` mit dem Feature `automation`, beobachtet den dynamischen UI-Zustand und fährt den Child geordnet herunter.
10. `bevy_example` ohne `automation` hängt nicht von `automation_control` ab.

Verwende für Reflection- und Pointer-Systemtests eine kleine App ohne OS-Eingabeinjektion. Der gerenderte End-to-End-Test darf einen verfügbaren Display- oder Render-Adapter voraussetzen, muss diese Voraussetzung aber klar melden. Die übrigen Crate-Tests bleiben displayfrei.

Führe mindestens aus:

```bash
cargo fmt --all --check
cargo check -p bevy_example
cargo check -p bevy_example --features automation
cargo test -p automation_control
cargo test -p automation_control --features driver
```

Führe außerdem den scripted Host-Pfad aus, wenn die lokale Render-Umgebung verfügbar ist. Prüfe den Player-Run-Abhängigkeitsbaum und weise nach, dass `automation_control` ohne aktiviertes Feature nicht im normalen App-Build landet.

Dieser Schritt ist abgeschlossen, wenn alle displayfreien Checks grün sind, der scripted Host-Pfad entweder erfolgreich gelaufen ist oder mit einer konkreten Umgebungsursache als nicht ausführbar dokumentiert wurde und keine öffentliche Schnittstelle mehr von der gelöschten Target-/Action-Architektur abhängt.

## Stop-Regeln

Stoppe und frage nach, wenn eine Entscheidung nötig wird, die einen der folgenden Punkte verändert:

- Controlled Session und Player Run werden nicht mehr als getrennte Build-Kompositionen behandelt.
- Reflection erhält Schreibzugriff.
- Bevy-Entity-Handles sollen über Sessions hinweg stabil werden.
- Native OS-Eingaben sollen doch in eine Controlled Session gelangen.
- Der erste Schnitt soll Tastatur, Recording, Replay, Screenshot oder mehrere Instanzen einschließen.
- Eine Änderung an `apps/app` wäre nötig, die über Compile-Kompatibilität oder die optionale Automation-Komposition hinausgeht.

Treffe lokale Modul-, Fehlercode- und Serialisierungsentscheidungen selbst, solange sie die festgelegte Schnittstelle erhalten und durch Tests belegt sind.

## Abschlussbericht

Berichte am Ende:

- geänderte und entfernte Dateien,
- die endgültige öffentliche Protokollform mit je einem JSON-Beispiel für Observe und Pointer,
- welche Daten Reflection direkt liefert und welche kontextuell abgeleitet werden,
- ausgeführte Befehle und Ergebnisse,
- Status des gerenderten Host-Smoke-Tests,
- verbleibende Risiken und bewusst vertagte Teile aus Issue #49.

Erstelle keinen Commit, sofern der Benutzer ihn nicht ausdrücklich verlangt.
