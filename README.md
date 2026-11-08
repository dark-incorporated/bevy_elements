# bevy_elements

> ⚠️ **Vibe Coded.** Written by AI agents working from a human's direction, covered by tests rather than a line-by-line human review. Read the code before you rely on it for something you care about.

Living environments for Bevy 0.20: a sun and moon on real ephemerides, the bright-star sky, a seeded climate that schedules hourly weather, a GPU wind field, volumetric clouds, rain and snow, lightning and thunder, fog, heightfield terrain with erosion and flooding, ground cover, rocks, dust, paths and trails — all in compute shaders, for anyone building an open world that has to look alive without a weather artist.

[![crates.io](https://img.shields.io/crates/v/bevy_elements.svg)](https://crates.io/crates/bevy_elements)
[![docs.rs](https://docs.rs/bevy_elements/badge.svg)](https://docs.rs/bevy_elements)
[![CI](https://github.com/dark-incorporated/bevy_elements/actions/workflows/ci.yml/badge.svg)](https://github.com/dark-incorporated/bevy_elements/actions/workflows/ci.yml)
[![Bevy](https://img.shields.io/badge/bevy-0.20-blue)](https://bevyengine.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#who-and-licence)

![The sky over one morning: a clear sky, then a broken cumulus deck, then full stratus cover, then fog, the preset named on screen and the HUD printing the temperature, cover, wind and visibility](https://raw.githubusercontent.com/dark-incorporated/bevy_elements/main/docs/media/tour_hero.gif)

*`weather_tour` forcing `WeatherPreset::Clear`, `PartlyCloudy`, `Overcast` and `Fog` in turn at 10.00 h on 20 April 2026 — the first three hours of its tour of all ten presets.*

**Status:** usable · Bevy 0.20 · MSRV 1.97.1

**Why:** the obvious alternative is a sky shader plus a rain particle system, which gives you weather but not a world — nothing to ask about the ground, the water, the light or the wind, and nothing that survives a save. This crate keeps one simulation behind an `Environment` query, so gameplay reads the weather instead of guessing at it.

## Quick start

```sh
cargo add bevy_elements
```

```rust,no_run
use bevy::prelude::*;
use bevy_elements::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ElementsPlugins))
        .insert_resource(WorldClock::new(CivilDate::new(2026, 6, 21)).at_hour(7.0))
        .insert_resource(Climate::temperate())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // The environment follows and renders for the `ElementsCamera`.
    commands.spawn((
        Camera3d::default(),
        ElementsCamera::default(),
        Transform::from_xyz(0.0, 160.0, 0.0).looking_at(Vec3::new(100.0, 100.0, 0.0), Vec3::Y),
    ));
    // 2 km of rolling hills, centred on the origin.
    commands.spawn(Terrain::procedural(ProceduralTerrain::hills(42)));
}
```

## Try it

```sh
cargo run --release --example quickstart
```

The example lifts the camera to 6 m above the ground and holds it there while the clock runs; add `ELEMENTS_FRAMES=600 ELEMENTS_SHOT=/tmp/x.png` to capture it unattended.

## What you can do

- **Put a living sky over any scene** — sun, moon and 8 404 stars from real ephemerides, with exposure that keeps night dark: [sky, weather and light](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/sky-weather.md).
- **Let a seeded climate schedule the weather** — `Clear` through `Blizzard`, hourly, with seasons and a snow season a game sets by date: [climate, seasons and weather](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/sky-weather.md#climate-seasons-and-weather).
- **Rain, snow, fog and lightning that read from every camera** — including an orthographic, isometric or strategy view, with `PrecipitationLook::readable()` for far-off cameras: [precipitation](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/sky-weather.md#precipitation).
- **Generate, sculpt and save a terrain** — procedural presets, a height map or flat ground, brushes that edit the live heightfield, snapshots to save and restore it: [terrain](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/terrain.md).
- **Wear the ground down realistically** — snow that drifts and melts, wetness and puddles, wind-blown sand, trodden paths and pressed footprints: [ground cover](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/terrain.md#ground-cover), [paths](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/terrain.md#paths-and-roads).
- **Flood it, and answer what the water is doing** — shallow-water hydrology with erosion, lakes, rivers, and wakes from anything moving through: [rivers, lakes and waves](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/water.md).
- **Ask the simulation anything** — ground height and normal, water depth and velocity, snow depth, cloud shadow, light needed for a night lamp: [gameplay queries](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/queries.md).

![An isometric town 1500 m from its camera, which follows the walker: rain, snow, fog and a partly cloudy afternoon](https://raw.githubusercontent.com/dark-incorporated/bevy_elements/main/docs/media/isometric.png)

![Clear at 10.00 h on 20 April: cover 0.05, 40 000 m visibility, wind 3.0 m/s, the sun 40.1° up](https://raw.githubusercontent.com/dark-incorporated/bevy_elements/main/docs/media/still.png)

*The `isometric` example: an orthographic camera 1 500 m back, following a walker, in the crate's rain, snow and fog.*

## Part of Dark Inc.

bevy_elements is one crate in [Dark Inc.](https://github.com/dark-incorporated), a family of Bevy crates that cover the parts of a game only human authoring scales: world, life, interface and spectacle. In [the Dark Factory](https://dark-incorporated.github.io/crates/bevy_elements.html#use-it-in-the-dark-factory) it is an optional cargo feature whose map environment panel edits this crate's own serde settings — the clock, the climate, the forced weather, snow, wetness, rocks and dust.

## Bevy compatibility

| bevy_elements | bevy | notes |
|---|---|---|
| 0.7 | 0.20 | current |
| 0.1 – 0.6 | 0.20 | `isomesh` needs `isomesh`/`bevy_isomesh` 0.0.11; `flora` needs `bevy_flora` 0.46 |

## Learn more

- [docs.rs](https://docs.rs/bevy_elements) — the full API.
- [The book page](https://dark-incorporated.github.io/crates/bevy_elements.html) — the crate in Dark Inc.'s book.
- [`docs/guide/`](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/README.md) — every domain in depth: [the scene](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/scene.md), [sky, weather and light](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/sky-weather.md), [terrain and ground](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/terrain.md), [rivers, lakes and waves](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/water.md), [surfaces and voxel snow](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/surfaces.md), [gameplay queries](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/queries.md), [performance](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/performance.md), [examples](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/examples.md), [research](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/guide/research.md).
- [`docs/gallery.md`](https://github.com/dark-incorporated/bevy_elements/blob/main/docs/gallery.md) — every before-and-after capture, the terrain's rock and the ground's textures.
- [`examples/`](https://github.com/dark-incorporated/bevy_elements/tree/main/examples) — 23 runnable examples, from `quickstart` to `bench_elements`.
- [`CHANGELOG.md`](https://github.com/dark-incorporated/bevy_elements/blob/main/CHANGELOG.md) — every release, with the research note behind it.
- [`docs/research/`](https://github.com/dark-incorporated/bevy_elements/tree/main/docs/research) — the paper library each system was built from, with every deviation cited.

## Who and licence

Maintained by [@Ladvien](https://github.com/Ladvien). Questions and bugs: [the issue tracker](https://github.com/dark-incorporated/bevy_elements/issues). Contributions follow the org's [`CONTRIBUTING.md`](https://github.com/dark-incorporated/.github/blob/main/CONTRIBUTING.md); docs follow [`DOCUMENTATION.md`](https://github.com/dark-incorporated/.github/blob/main/DOCUMENTATION.md).

MIT OR Apache-2.0, at your option.
