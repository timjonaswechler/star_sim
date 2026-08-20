# Research map for the plate tectonics prototype

## Canonical source

Worldbuilding Pasta, "An Apple Pie From Scratch, Part Va: Tectonics: Constructing a Plate Tectonic History"

<https://worldbuildingpasta.blogspot.com/2020/01/an-apple-pie-from-scratch-part-va.html>

The article is the implementation brief. It mixes established observations, simplified design rules, and mechanisms whose causes remain unsettled. Preserve that distinction in code. A deterministic scenario rule may represent an uncertain mechanism, but the event log must name it as a heuristic.

## Existing export artifacts

A prior chat exported and reviewed the complete article. Use these local files if they still exist:

| Path | Contents |
| --- | --- |
| `/tmp/apppie/post.html` | Raw exported page, about 665 KB. |
| `/tmp/apppie/post.md` | Article body converted to plain Markdown, 2,638 lines and about 138 KB. |
| `/tmp/apppie/section_images.txt` | The 107 image URLs at or after "Patterns of Plate Motion", in article order. |
| `/tmp/apppie/image_map.txt` | Image number, source filename, and nearby text for all 107 images. |
| `/tmp/apppie/imgs/` | Downloaded source images. |
| `/tmp/apppie/contact_00.png` through `contact_06.png` | Contact sheets for all still images. Red labels match `image_map.txt`. |
| `/tmp/apppie/contact_gif_007.png` | Twelve sampled frames from the 38-frame Pangea breakup GIF. |

These are temporary files. If they are gone, fetch the canonical URL again. Start at the HTML anchor `#patternsofplatemotion`, retain headings and captions, and download every following `img` link. Inspect animated files by sampling their frames.

## Article map

Line numbers refer to `/tmp/apppie/post.md`.

| Lines | Material |
| --- | --- |
| 96-360 | "Earth Today": crust, lithosphere, mantle, ridge formation, crust aging, subduction, volcanism, continental growth, and the three boundary types. |
| 363-429 | Supercontinent cycle overview. |
| 430-617 | Extroversion. |
| 618-814 | Introversion, Wilson cycles, and orthoversion. |
| 815-861 | Flat-slab subduction. |
| 862-926 | Slab rollback and back-arc extension. |
| 927-968 | Island-arc accretion. |
| 969-1031 | Subduction jumping and polarity reversal. |
| 1032-1057 | Rotation and arc formation. |
| 1058-1098 | Subduction invasion. |
| 1099-1152 | Triple-junction ocean-plate formation. |
| 1153-1184 | Continental megashear. |
| 1185-1240 | Tethys-type oceans. |
| 1241-1272 | Complex collisions. |
| 1273-1395 | Plate speeds, crust growth, and sea level. |
| 1396-1611 | Simulation approaches and a hand-built quick method. |
| 1612-2471 | An 850-million-year GPlates example. |
| 2472-2541 | Article summary. |

## Complete image map

Every image at or after "Patterns of Plate Motion" belongs to one of these groups. This grouping accounts for all 107 exports.

| Images | What to inspect |
| --- | --- |
| 001 | Cross-sections of extroversion, introversion, and related global cycles. |
| 002-014 | Extroversion sequence: stable supercontinent, plume and triple-arm rifts, connected zigzag rift, failed arms, Pangea breakup animation, spreading, active and passive margins, interior and exterior oceans, ridge drift, ridge subduction, collision, suture. Image 007 is the GIF. |
| 015-024 | Subduction initiation modes, trench propagation, re-rifting, inherited mountains, one-sided interior subduction, Wilson cycle, orthoversion, and four proposed future supercontinents. |
| 025-029 | Flat slab, ridge subduction and slab window, rollback, back-arc basin, leading and trailing active margins. |
| 030-037 | Offshore arc formation, arc accretion, microcontinent collision, subduction jump, polarity reversal, and two-sided consumption of a small ocean plate. |
| 038-039 | A trench bridging rifting continents and hinge-like rotation producing an arc. |
| 040-042 | Subduction invasion through transform collapse or collision, followed by trench propagation into an interior ocean. |
| 043-045 | Triple-ridge junction, rotational gap, and growth of a new central ocean plate while old plates shrink. |
| 046 | Proposed Pangea B to Pangea A continental megashear. |
| 047-050 | Tethys-type basin, tearing and transport of a microcontinent, trench jump, and final broad plateau collision. |
| 051-052 | Irregular continental collision and a Mediterranean-like result with enclosed seas, rollback, islands, and local orogenies. |
| 053 | Example generated elevation map. It is output inspiration, not a mechanics rule. |
| 054-061 | Quick method: rift a supercontinent, rotate fragments, assign margins, place inherited and active mountains, then assemble by extroversion or introversion. |
| 062-103 | GPlates history from 850 million years ago to the modeled present. Alternating Mollweide maps and orthographic close-ups show cratons, crust ages, plate boundaries, velocity arrows, microcontinents, ocean plates, orogenies, large igneous provinces, hotspots, and polar projection effects. |
| 104 | Geological period and continent-membership chart. |
| 105 | Final continent and ocean names. |
| 106 | Orogeny age map. |
| 107 | Final topography derived from the recorded tectonic history. |

The numbered mechanics diagrams use a consistent legend:

- Red boundary: divergent rift or mid-ocean ridge.
- Green boundary: transform or transverse motion.
- Blue boundary with teeth: convergence or subduction. Teeth point toward the overriding plate.
- Black arrows: plate motion.
- Brown: mountain building.
- Maroon dots: volcanism.
- Plan view is above a simplified crustal cross-section.

The detailed GPlates sequence adds black orogenies, orange large igneous provinces, purple hotspots, gray cratons, and blue shades for ocean-crust age.

## Foundation model

### Spherical rigid motion

A plate moves as a rigid rotation about an Euler pole. Store its angular-velocity vector `omega` in radians per unit time. For a unit surface point `p` and planet radius `R`:

```text
position = R * p
velocity = omega x position
```

Update points with an axis-angle rotation derived from `omega * dt`, then renormalize. This naturally produces rotation, pole crossings, and projection distortion without flat translations.

At a sampled boundary point, define a unit boundary tangent `t` in the local tangent plane. Define the oriented in-surface normal as `n = normalize(p x t)`. For left and right plate velocities:

```text
relative = velocity_right - velocity_left
normal_rate = dot(relative, n)
shear_rate = dot(relative, t)
```

Use the signed normal rate for convergence or divergence and the tangential rate for transform motion. A mixed boundary may change class along its length. This is required by the article and visible throughout the GPlates close-ups.

### Crust and boundaries

- A ridge creates ocean crust on both sides and attaches each side to its adjacent plate.
- Symmetric spreading makes ridge motion follow the midpoint of adjacent surface velocities. A static plate beside a plate moving at 10 cm/year makes the ridge drift at about 5 cm/year in the moving direction.
- Ocean crust cools, sinks, stiffens, and becomes denser with age. Age affects slab pull and subduction preference. Age alone never deletes crust.
- A trench consumes ocean-crust markers from the subducting plate. The overriding side keeps or accretes buoyant material.
- Continental crust is thick, buoyant, brittle, and persistent. It can stretch, tear, slide, compress, accrete, and partially underthrust, but a large continent does not sink into the mantle.
- A collision between large continents stops the affected trench, detaches the oceanic slab, creates a suture, and joins their subsequent rigid motion.
- A thin microcontinent or ocean plateau can interrupt a trench and allow it to jump to the far side.

### Drivers

The article's central rule is that subduction drives plate motion and ridge spreading responds to it.

For a prototype, convert each sampled line force `f` at `p` into torque:

```text
torque = (R * p) x f
```

Sum torques, apply an effective rotational inertia, then damp angular velocity. The terms are:

- Slab pull: strongest term, applied to the subducting plate toward the trench. Increase it with the age and length of subducting ocean crust.
- Slab suction: weaker term pulling the overriding plate toward the trench.
- Ridge push: secondary force away from a ridge. Increase it during plume-uplifted supercontinent breakup, but do not let it drive an otherwise unsupported continent indefinitely.
- Mantle drag: opposes angular velocity. Increase drag with continental area or continental root depth.

These are a kinematic abstraction. Coefficients should produce the article's speed classes rather than claim physical force units.

### Speed scale

The article gives these working ranges:

| Plate situation | Speed |
| --- | --- |
| Subducting ocean plate | 10-20 cm/year |
| Plate after a recent subduction collision | 5-10 cm/year |
| Active-margin continent | 2-5 cm/year |
| Passive-margin continent | Less than 1 cm/year |

On an Earth-radius planet, 1 cm/year equals about 10 km per million years, 0.09 degrees per million years, or 0.9 degrees per 10 million years. Scale angular speed inversely with planet radius when using the same linear speed.

## Mechanic rules

### Supercontinent breakup

A supercontinent insulates or reorganizes mantle heat. Plumes uplift and weaken its interior. Peripheral trenches supply the tensile force. Initial rifts tend to form three arms about 120 degrees apart. Arms from neighboring junctions connect into a zigzag main rift; unmatched third arms stop as failed rifts. Failed rifts remain weak, low regions and can reactivate later. Breakup is staged and fragments rotate around different Euler poles.

Outputs include plume or large-igneous-province markers, active rifts, failed rifts, new ridges, transforms, inherited matching margins, and passive coasts.

### Extroversion, introversion, and orthoversion

- Extroversion keeps fragments moving outward until the old exterior ocean closes on the far side. The new interior ocean survives.
- Introversion starts subduction in the interior ocean. One-sided consumption captures its ridge and reverses relative continental motion. The interior ocean closes and the exterior ocean survives.
- Orthoversion rotates fragments about a quarter-turn so assembly closes parts of both oceans.

Real histories combine these basin by basin. Scenarios should model the ideal patterns rather than force one label on a whole planet.

### Subduction initiation and propagation

The article lists three broad starts:

- Passive-margin collapse along inherited weakness.
- Transform collapse where older, denser crust lies beside younger crust.
- Plume-head margin collapse around uplifted crust.

Collision-driven polarity reversal and trench jumping are induced starts. Once a trench exists, slab pull stresses neighboring faults and propagates the trench laterally. Implement starts as explicit threshold events using crust age, fault type, buoyancy contrast, and local convergence. Record which rule fired.

### Flat-slab subduction

Two proposed causes are fast overriding-plate motion and subduction of young buoyant crust or a ridge. Reduce slab dip when trench-normal overriding speed is high or slab buoyancy is high. Lower dip broadens inland compression and uplift. Move volcanic activity inland; turn it off where the flat slab removes the mantle wedge. Ridge capture may break the slab and create a slab-window marker.

### Slab rollback

Slow crust supply to a trench means progressively older, denser crust reaches it. The slab steepens and the trench retreats toward the ocean. Slab suction stretches the overriding edge. Move the arc with the trench and open a back-arc basin behind it. Weak fractures may form volcanic islands. If overriding motion later catches the retreating trench, close the basin and accrete its material onto the continent.

### Island-arc growth and accretion

Subduction releases volatiles and creates felsic volcanic crust on the overriding side. Place an arc landward of and roughly parallel to the trench. If another trench later consumes the ocean between the arc and a continent, collide the arc with the continent, create a Ural-type collision belt, add the arc's continental material, and move the active margin seaward.

### Trench jump and polarity reversal

A microcontinent collision closes the incoming trench locally. If the colliding block is thin, fracture old ocean crust on its far side and start a same-polarity trench there. A large continent stops subduction.

For polarity reversal, an arc initially overrides ocean crust attached to an approaching continent. After collision, start the replacement trench on the far side of the arc-continent assemblage. The former subducting side becomes overriding.

When two trenches consume the same small ocean plate, let age, buoyancy, and imposed scenario asymmetry decide which trench survives after collision. The article says no universal rule is known.

### Rotation and arc formation

Uneven boundary torques make plates separate like hinges. A trench intersected by a rift often propagates across the opening instead of splitting. Advect its sampled points with the adjacent plates and insert points as the gap grows. Rigid motion on the sphere and differential motion should produce the arc shape. Avoid a flat circular-arc drawing shortcut.

### Subduction invasion

A gateway between exterior and interior oceans may hold a transform boundary or a landmass collision. Transform collapse or polarity reversal starts a trench there. Propagate it into the interior ocean along stressed faults and then onto nearby continental margins. This mechanism can turn an extroversion history toward introversion.

### Triple-junction plate formation

Three ridges normally add crust to their three existing plates. If their relative motion has enough rotational shear around the junction, a gap can open that belongs to none of them. Create a new ocean plate from that gap and surround it with ridges. The new plate stays comparatively slow while old plates move toward peripheral trenches. The surrounding ridges migrate outward; old plates and their ridges may be consumed, leaving no crust source and destabilizing the ocean basin.

Treat the gap trigger as a documented kinematic threshold. The article refers to Boschman and Van Hinsbergen 2016 for the more complicated Pacific-birth geometry.

### Continental megashear

Apply mostly tangential relative velocity along a long internal continental boundary. The two blocks slide thousands of kilometers while retaining a joined supercontinent. The Pangea B interpretation is disputed, so label this scenario as a plausibility demonstration rather than an expected cycle stage.

### Tethys-type basin

Put a trench on the inside of an arcuate continent. Its pull acts on the far shore, but the connecting continental arc prevents direct collision. Tear a brittle fragment from that shore and carry it toward the trench. Small fragments accrete and permit trench jumps. Repeat if ocean crust remains. A final large continent stops subduction. Repeated small collisions leave weak belts; the final rapid collision reactivates them and creates a broad Himalayan-type plateau.

### Complex collision

Sample irregular continental margins. Let protrusions collide before embayments. Contacted samples become sutures while open samples retain ocean, ridges, or trenches. Local rollback can move a trench into another landmass and stretch an enclosed back-arc basin. The result should contain small plates, islands, peninsulas, orogenies, and trapped ocean pockets rather than one simultaneous collision line.

## Recorded features and secondary outputs

These outputs support the scenarios but need not drive plate motion.

### Orogenies

- Andean type: coastal subduction volcanism, about 80-200 km wide in the examples.
- Laramide type: flat slab or ridge subduction, about 750-1,300 km wide.
- Ural type: direct continent, microcontinent, or arc collision, about 50-180 km in simple examples.
- Himalayan type: repeated accretion or rapid one-sided continental collision, around 1,200 km wide.

The article's final terrain pass starts average mountain elevation near 2,500 m and reduces it by about 5 m per million years. It starts peaks near 4,000 m and reduces them by about 8 m per million years. It notes that exponential decay may be better. Keep this as optional output annotation.

### Hotspots and large igneous provinces

Hotspots stay mostly fixed relative to the mantle while plates move over them. Favor rift centers, antipodes of former supercontinents, and weakened back-arc crust, while allowing some elsewhere. Large igneous provinces mark short plume events, often near supercontinent rifts, and may leave hotspots behind.

### Sea level and continental area

Young ocean crust is buoyant and occupies more basin volume. After breakup, replacement of old crust plus continental movement off plume uplift tends to raise relative sea level. Aging ocean crust and supercontinent assembly lower it. The article quotes present continental growth near 17,000-26,000 square kilometers per million years, mostly through arc accretion. Both are optional diagnostics, not core motion rules.

## Lessons from the 850-million-year example

The detailed image sequence is not another required scenario. It verifies that the local rules must compose in these ways:

- Rifting occurs in stages and reuses failed rifts.
- Different subduction segments give one plate simultaneous translation-like drift and rotation.
- A trench may consume a ridge, split a continent, or isolate a new ocean-only plate.
- Microcontinents repeatedly tear off, travel quickly, collide, and trigger trench jumps.
- Island arcs accrete before larger continental collisions.
- One nominal global cycle mixes extroversion, introversion, and orthoversion in different basins.
- Ocean-only plates appear when trenches cut continents away from old ocean crust or when triple junctions open.
- A ridge too long for consistent spherical spreading reorganizes and gains a trench along part of its length.
- Relative motion near the poles looks misleading in Mollweide or equirectangular maps. Compute on the sphere and project only for display.
- Hotspots remain mantle-fixed while plate geometry passes over them.
- Final topography depends on the full orogeny history, not only current boundaries.

## Scientific boundary

The article repeatedly states that subduction initiation, trench jumping, polarity reversal, flat slabs, and Pacific-style plate birth are not fully understood. The prototype should answer whether a coherent state model can represent their observed sequence. It should not present its thresholds as a geodynamic prediction model.
