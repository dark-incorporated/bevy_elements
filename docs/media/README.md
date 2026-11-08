# Media

Every image and GIF the README, the [guide](../guide/README.md) and the [gallery](../gallery.md) use, with the command that regenerates it, the date it was captured and the crate version it shows.

Frames come from the crate's own capture path: `ELEMENTS_FRAMES=N` (a fixed 1/60 s step) with `ELEMENTS_SHOT=/path.png` for a still, or `ELEMENTS_SHOT_DIR=/dir ELEMENTS_SHOT_EVERY=N` for a frame sequence (`examples/common/mod.rs`). Examples are always run with `--release`. To make a GIF from a frame sequence:

```sh
ffmpeg -framerate 30 -i f_%04d.png -vf "fps=15,scale=960:-1:flags=lanczos,split[a][b];[a]palettegen=stats_mode=diff[p];[b][p]paletteuse=dither=bayer" -loop 0 out.gif
```

Every budget in the guide is met: each GIF is 3-8 s, at most 960 px wide, 15 fps and under 4 MB; `tour.gif` is 34 s and is the one exception, kept because it is the whole weather tour, and `tour_hero.gif` is a 6.8 s excerpt of it. GIFs are never in Git LFS: crates.io rewrites a relative image path to `https://github.com/dark-incorporated/bevy_elements/raw/HEAD/<path>`.

| File | Captured | Crate version | Regenerate |
|---|---|---|---|
| [`bed_colour.png`](bed_colour.png) | 2026-10-22 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`bedding.png`](bedding.png) | 2026-10-21 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`belt_of_venus.png`](belt_of_venus.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`block_sizes.png`](block_sizes.png) | 2026-10-22 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`clear.png`](clear.png) | 2026-10-13 | 0.4 or earlier | regeneration command unknown; captured before this list existed |
| [`cloud_lighting.png`](cloud_lighting.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`cloud_shadow_sky.png`](cloud_shadow_sky.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`cloud_sunset.png`](cloud_sunset.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`clouds.png`](clouds.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`cumulus.png`](cumulus.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`dawn.png`](dawn.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`dust_front_10s.png`](dust_front_10s.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`dust_front_70s.png`](dust_front_70s.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`dust_storm.png`](dust_storm.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`flora_wind.png`](flora_wind.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`fog.png`](fog.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`game_look.gif`](game_look.gif) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`game_look.png`](game_look.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`ground_colour.png`](ground_colour.png) | 2026-10-20 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`ground_colour_hills.png`](ground_colour_hills.png) | 2026-10-20 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`ground_cover_rain.png`](ground_cover_rain.png) | 2026-10-23 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`ground_cover_snow.png`](ground_cover_snow.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`ground_detail.png`](ground_detail.png) | 2026-10-19 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`ground_textures.png`](ground_textures.png) | 2026-10-23 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`isometric.gif`](isometric.gif) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`isometric.png`](isometric.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`night.png`](night.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`night_sky.png`](night_sky.png) | 2026-10-20 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`night_sky_colour.png`](night_sky_colour.png) | 2026-10-20 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`night_town.png`](night_town.png) | 2026-10-20 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`noon.png`](noon.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`orthographic_lod.png`](orthographic_lod.png) | 2026-10-25 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`overcast.png`](overcast.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`paths.png`](paths.png) | 2026-10-26 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`precipitation_look.gif`](precipitation_look.gif) | 2026-10-27 | 0.6.0 | regeneration command unknown; captured before this list existed |
| [`precipitation_look.png`](precipitation_look.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`rain.png`](rain.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`rock_blocks.png`](rock_blocks.png) | 2026-10-21 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`rock_facets.png`](rock_facets.png) | 2026-10-26 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`rock_fragments.png`](rock_fragments.png) | 2026-10-21 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`rock_material.png`](rock_material.png) | 2026-10-20 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`rock_mountain.png`](rock_mountain.png) | 2026-10-20 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`rock_palette.png`](rock_palette.png) | 2026-10-21 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`rocks.png`](rocks.png) | 2026-10-22 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`rocks_orthographic.png`](rocks_orthographic.png) | 2026-10-26 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`scree.png`](scree.png) | 2026-10-21 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`sky_light.gif`](sky_light.gif) | 2026-10-27 | 0.6.0 | regeneration command unknown; captured before this list existed |
| [`sky_light.png`](sky_light.png) | 2026-10-20 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`snow_season.gif`](snow_season.gif) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`snow_season.png`](snow_season.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`snowfall.png`](snowfall.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`still.png`](still.png) | 2026-10-31 | 0.7.6 | the hero GIF's first frame: `ffmpeg -i docs/media/tour_hero.gif -vf "select=eq(n\,0)" -frames:v 1 docs/media/still.png` |
| [`storm.png`](storm.png) | 2026-10-13 | 0.4 or earlier | regeneration command unknown; captured before this list existed |
| [`sunset.png`](sunset.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`tour.gif`](tour.gif) | 2026-10-18 | 0.5.3 | `ELEMENTS_SHOT_DIR=/tmp/tour ELEMENTS_FRAMES=12600 ELEMENTS_SHOT_EVERY=48 ELEMENTS_SHOT_FROM=36 cargo run --release --example weather_tour`, then `magick -delay 13 -loop 0 /tmp/tour/frame*.png -resize 512x -colors 200 -layers Optimize docs/media/tour.gif` |
| [`tour_hero.gif`](tour_hero.gif) | 2026-10-31 | 0.7.6 | the tour's first four presets (Clear, PartlyCloudy, Overcast, Fog) cut from `docs/media/tour.gif`: `ffmpeg -i docs/media/tour.gif -vf "select='between(n,8,85)',scale=512:-1:flags=lanczos,setpts=N/(12*TB),split[a][b];[a]palettegen=stats_mode=diff[p];[b][p]paletteuse=dither=bayer" -loop 0 docs/media/tour_hero.gif` |
| [`trails.gif`](trails.gif) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`trails_game_ground.gif`](trails_game_ground.gif) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`trails_game_ground.png`](trails_game_ground.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`trails_mud.png`](trails_mud.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`trails_sand.png`](trails_sand.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`trails_snow.png`](trails_snow.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`twilight.png`](twilight.png) | 2026-10-17 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`units.png`](units.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`voxel_snow.png`](voxel_snow.png) | 2026-10-14 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`voxel_walls.gif`](voxel_walls.gif) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`voxel_walls.png`](voxel_walls.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`water_flood.png`](water_flood.png) | 2026-10-15 | 0.5.0 | regeneration command unknown; captured before this list existed |
| [`water_flood_rain.png`](water_flood_rain.png) | 2026-10-23 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`water_lake.png`](water_lake.png) | 2026-10-13 | 0.4 or earlier | regeneration command unknown; captured before this list existed |
| [`water_still.png`](water_still.png) | 2026-10-27 | 0.6.0 | regeneration command unknown; captured before this list existed |
| [`waves.gif`](waves.gif) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`waves_rings.png`](waves_rings.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |
| [`waves_wake.png`](waves_wake.png) | 2026-10-18 | 0.5.3 | regeneration command unknown; captured before this list existed |

## Copies at the old paths

Until 2026-10-31 these files lived directly under `docs/` (and some under other `docs/` folders). Every release published before then has a README on crates.io and docs.rs whose image URLs name those old paths, and a published README never changes. So each moved file also stays at its old path, as an identical copy: git stores the same content once, so a copy costs nothing in the repository's size. **Do not delete them as duplicates**; when one of the files here is re-recorded, the old-path copy is updated with it (`docs/media/` is the one new references use). The org's rule is in [DOCUMENTATION.md](https://github.com/dark-incorporated/.github/blob/main/DOCUMENTATION.md#image-and-link-urls-in-a-readme).
