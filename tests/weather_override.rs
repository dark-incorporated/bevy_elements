//! `WeatherOverrideSettings`: a project's forced weather reaches the director. Headless, no GPU.

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;
use bevy_elements::*;

fn app(settings: Option<WeatherOverrideSettings>) -> App {
    let mut app = App::new();
    // Inserted before the plugin, so the first frame sees it as added.
    if let Some(settings) = settings {
        app.insert_resource(settings);
    }
    app.add_plugins((MinimalPlugins, ClimatePlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            100,
        )))
        .insert_resource(WorldClock::new(CivilDate::new(2026, 6, 21)).at_hour(12.0));
    app
}

fn forced(app: &App) -> Option<WeatherPreset> {
    app.world().resource::<WeatherDirector>().forced()
}

#[test]
fn a_forced_preset_reaches_the_director_and_none_releases_it() {
    let mut app = app(None);
    app.update();
    assert_eq!(forced(&app), None, "the default forces nothing");

    app.world_mut()
        .resource_mut::<WeatherOverrideSettings>()
        .preset = Some(WeatherPreset::Rain);
    app.update();
    assert_eq!(forced(&app), Some(WeatherPreset::Rain));
    assert!(app.world().resource::<WeatherDirector>().is_forced());

    app.world_mut()
        .resource_mut::<WeatherOverrideSettings>()
        .preset = None;
    app.update();
    assert_eq!(forced(&app), None, "None released the director");
}

#[test]
fn a_preset_present_on_the_first_frame_is_forced() {
    let mut app = app(Some(WeatherOverrideSettings {
        preset: Some(WeatherPreset::Snow),
    }));
    app.update();
    assert_eq!(forced(&app), Some(WeatherPreset::Snow));
}

#[test]
fn the_default_on_the_first_frame_leaves_a_restored_forced_director_alone() {
    let mut app = app(None);
    app.world_mut()
        .resource_mut::<WeatherDirector>()
        .force(WeatherPreset::Fog, Duration::from_secs(1));
    app.update();
    assert_eq!(
        forced(&app),
        Some(WeatherPreset::Fog),
        "a default `None` on the first frame must not release a forced director"
    );
}

#[test]
fn an_unchanged_setting_does_not_restart_a_blend() {
    let mut app = app(Some(WeatherOverrideSettings {
        preset: Some(WeatherPreset::Rain),
    }));
    app.update();
    // The game forces a different preset itself; the unchanged setting leaves it alone.
    app.world_mut()
        .resource_mut::<WeatherDirector>()
        .force(WeatherPreset::Fog, Duration::from_secs(1));
    app.update();
    assert_eq!(forced(&app), Some(WeatherPreset::Fog));
}
