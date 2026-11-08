//! Climate, seasons and weather: a seeded scheduler that picks hourly weather from a
//! [`Climate`], a director that lets the game force it, and the ground's response.

mod history;
mod model;
mod season;
mod weather;
pub(crate) mod weathering;

use bevy::prelude::*;

pub use history::{ClimateHistory, ClimateSummary, DayConditions, FREEZING_C, WET_DAY_MM};
pub use model::{Climate, LAPSE_RATE_C_PER_M, SnowSeason};
pub use season::{Season, SeasonState};
pub use weather::{Weather, WeatherChanged, WeatherDirector, WeatherPreset};
pub(crate) use weather::{hash64, mix64};
pub use weathering::SurfaceWeathering;

use crate::celestial::{GeoLocation, WorldClock};
use crate::plugin::{ElementsSystems, ensure_core};

/// The season changed. Also written once on the first frame, so a reader can sync without
/// polling [`SeasonState`].
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SeasonChanged {
    /// The season now.
    pub season: Season,
}

/// The game time a forced or released weather takes to blend, in seconds: ten minutes, as
/// [`WeatherOverrideSettings`] applies it.
pub const WEATHER_OVERRIDE_BLEND_S: f32 = 10.0 * 60.0;

/// The weather a project forces. `None` lets the climate's scheduler choose.
///
/// A settings resource: a project saves it, and [`apply_weather_override`] hands a change to the
/// [`WeatherDirector`] in [`ElementsSystems::Weather`], blending over
/// [`WEATHER_OVERRIDE_BLEND_S`] seconds of game time. `Some(preset)` forces the preset;
/// changing it back to `None` releases the director to the scheduler. The first frame applies the
/// resource only when it holds a preset, so a restored [`WeatherDirector`] that is already forced
/// is not released by the default `None`.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
#[reflect(Resource, Default)]
#[cfg_attr(feature = "serde", reflect(Serialize, Deserialize))]
pub struct WeatherOverrideSettings {
    /// The preset to hold, or `None` to let the scheduler choose.
    pub preset: Option<WeatherPreset>,
}

/// Force or release the [`WeatherDirector`] when [`WeatherOverrideSettings`] changes.
///
/// Runs first in the climate chain, so the request is carried out the same frame.
pub fn apply_weather_override(
    settings: Res<WeatherOverrideSettings>,
    mut director: ResMut<WeatherDirector>,
) {
    if !settings.is_changed() || (settings.is_added() && settings.preset.is_none()) {
        return;
    }
    let blend = std::time::Duration::from_secs_f32(WEATHER_OVERRIDE_BLEND_S);
    match settings.preset {
        Some(preset) => director.force(preset, blend),
        None if director.is_forced() => director.release(blend),
        None => {}
    }
}

/// Climate, seasons, weather, surface weathering and the record of recent days.
///
/// Inserts (if absent) [`Climate`], [`WeatherDirector`], [`SeasonState`],
/// [`SurfaceWeathering`], [`ClimateHistory`] and [`WeatherOverrideSettings`], and writes
/// [`Weather`] every frame in [`ElementsSystems::Weather`].
pub struct ClimatePlugin;

impl Plugin for ClimatePlugin {
    fn build(&self, app: &mut App) {
        ensure_core(app);
        app.init_resource::<Climate>()
            .init_resource::<WeatherDirector>()
            .init_resource::<SeasonState>()
            .init_resource::<SurfaceWeathering>()
            .init_resource::<ClimateHistory>()
            .init_resource::<WeatherOverrideSettings>()
            .register_type::<Climate>()
            .register_type::<WeatherOverrideSettings>()
            .add_message::<WeatherChanged>()
            .add_message::<SeasonChanged>()
            .add_systems(
                Update,
                (
                    apply_weather_override,
                    update_season,
                    weather::run_weather,
                    weathering::update_weathering,
                    history::record_history,
                )
                    .chain()
                    .in_set(ElementsSystems::Weather),
            );
    }
}

fn update_season(
    clock: Res<WorldClock>,
    location: Res<GeoLocation>,
    mut state: ResMut<SeasonState>,
    mut changed: MessageWriter<SeasonChanged>,
    mut first: Local<bool>,
) {
    let days_in_year = if clock.date.is_leap_year() {
        366.0
    } else {
        365.0
    };
    let day = f32::from(clock.day_of_year()) + clock.hour as f32 / 24.0;
    let next = SeasonState::at(day, clock.date.month, location.is_southern(), days_in_year);
    if !*first || next.season != state.season {
        changed.write(SeasonChanged {
            season: next.season,
        });
        *first = true;
    }
    *state = next;
}
