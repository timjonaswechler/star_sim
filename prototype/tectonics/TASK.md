# TASK – Tectonics Prototype (`prototype/tectonics/`)

> Status: Draft – 2026-09-05.

## 1. Task

Build a prototype for the plate-motion patterns from Worldbuilding Pasta's
"An Apple Pie From Scratch, Part Va: Tectonics: Constructing a Plate Tectonic History".

The research map and this document are kept. The post and the extracted
contents live under `prototype/tectonics/source/`.

## 2. Canonical source

- Blog article: <https://worldbuildingpasta.blogspot.com/2020/01/an-apple-pie-from-scratch-part-va.html>
- The article mixes established observations, simplified design rules, and mechanisms
  with unsettled causes. Every future implementation must preserve this distinction:
  deterministic rules for uncertain mechanisms are allowed but must be labeled as
  **heuristic**.

## 3. Relevant files

All paths relative to `prototype/tectonics/`.

| File | Role | In Git? |
| --- | --- | --- |
| `TASK.md` (this document) | Task description, file list, goal. | Yes |
| `research.md` | Research map: article outline, image map (107 images), foundation model (Euler rotation, crust, drivers, speed scale), mechanics rules per mechanism, scientific boundary. | Yes |
| `source/post.md` | Article text from anchor `#patternsofplatemotion` as Markdown (~2,000 lines), image references as `[IMAGE nnn]` markers in article order. | Yes |
| `source/image_map.txt` | Image number, source filename, URL, and surrounding text for all 107 images. Red labels on the contact sheets match these numbers. | Yes |
| `source/section_images.txt` | The 107 image URLs in article order (download input). | Yes |
| `source/fetch_source.py` | Regenerates everything listed below from the canonical URL (needs `beautifulsoup4`, `Pillow`). | Yes |
| `source/post.html` | Raw article HTML (~650 KB). | No (`.gitignore`) |
| `source/imgs/` | The 107 downloaded images (`001.*` … `107.*`), incl. Pangea GIF `007.gif` (38 frames). | No (`.gitignore`) |
| `source/contact_00.png` … `contact_06.png` | ~10–16 stills each as one labeled large image (red numbers = `image_map.txt`). | No (`.gitignore`) |
| `source/contact_gif_007.png` | 12 sampled frames of the Pangea animation (image 007). | No (`.gitignore`) |

Regenerate the local files:

```bash
python prototype/tectonics/source/fetch_source.py
```

## 4. Goal (done criteria)

**Core question:** Can a simple kinematic state model (Euler rotation on the
sphere + crust age + explicit heuristic events, no geodynamics) reproduce all
mechanisms described in the blog as running sequences?

The prototype is done when a Python scenario runs for every blog mechanism
and visibly reproduces the mechanism sequence: supercontinent breakup
(triple junction, zigzag rift, failed rifts), extroversion / introversion /
orthoversion, subduction initiation + lateral propagation, flat slab,
slab rollback + back-arc, island-arc growth + accretion, trench jump,
polarity reversal, rotation + arc formation, subduction invasion,
triple-junction plate formation, megashear (labeled as a disputed
plausibility demo), Tethys type, complex collision.

**Rendering:** Every scenario renders in the style of the author's diagram series:
plan view (red = divergent, green = transverse, blue with teeth = convergent/subduction,
arrows = plate motion, brown = mountains, maroon = volcanism) above a
simplified crustal cross-section (thick light continental vs. thin
oceanic crust). External references (e.g. Bradley, Stern/Gerya, Niu, Davies)
and the GPlates 850-Ma sequence are references, not scenarios.

**Acceptance:**

5. Per mechanism: one runnable scenario (`scenario_<name>.py` or equivalent)
   + before/after render + event log naming every uncertain rule as
   `heuristic` (see `research.md`, section `Scientific boundary`).
6. Non-goals: no full planet generator, no geodynamic prediction model,
   no climate, no detailed erosion/topography beyond the orogen widths
   and decay rates in `research.md`.
