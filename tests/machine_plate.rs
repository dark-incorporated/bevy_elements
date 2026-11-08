//! The machine plate: the family API standard, checked against `ElementsPlugins`.
//!
//! Headless, no GPU: Bevy's default plugins with no window backend (`WgpuSettings::backends` is
//! `None`), as `bevy_gooey`'s tests run. The main world's schedules are checked; the render-world
//! systems carry `ElementsSystems::Render` in their `add_systems` calls and are covered by review
//! and by the GPU tests (`cargo test --test gpu_plugins -- --ignored`).
//!
//! Names come from `System::name()`, which is the full type path only with Bevy's `debug`
//! feature; the dev-dependency on `bevy` turns it on, and the test asserts it saw systems of the
//! crate so a build without it fails instead of passing vacuously.

use bevy::app::App;
use bevy::ecs::reflect::{AppTypeRegistry, ReflectComponent, ReflectResource};
use bevy::ecs::schedule::{Schedules, SystemSet};
use bevy::prelude::*;
use bevy::reflect::{GetTypeRegistration, TypeRegistry, prelude::ReflectDefault};
use bevy::render::settings::WgpuSettings;
use bevy::render::{RenderApp, RenderPlugin};
use bevy::winit::WinitPlugin;
use bevy_elements::*;

const CRATE: &str = "bevy_elements::";

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: None,
                    ..default()
                }
                .into(),
                ..default()
            })
            .disable::<WinitPlugin>(),
        ElementsPlugins,
    ));
    app.update();
    app
}

/// Every system of the crate must be in one of the crate's root-set variants.
#[test]
fn every_system_is_in_the_root_set() {
    let app = app();
    let roots = [
        ElementsSystems::Clock,
        ElementsSystems::Weather,
        ElementsSystems::Celestial,
        ElementsSystems::Simulation,
        ElementsSystems::Readback,
        ElementsSystems::Apply,
        ElementsSystems::Setup,
        ElementsSystems::Render,
    ]
    .map(|variant| variant.intern());
    // The main world, and the render world when this build has one.
    let worlds =
        std::iter::once(app.world()).chain(app.get_sub_app(RenderApp).map(|sub| sub.world()));
    let mut seen = 0;
    let mut outside = Vec::new();
    for world in worlds {
        let Some(schedules) = world.get_resource::<Schedules>() else {
            continue;
        };
        for (label, schedule) in schedules.iter() {
            let Ok(systems) = schedule.systems() else {
                continue;
            };
            for (key, system) in systems {
                let name = system.name().to_string();
                if !name.starts_with(CRATE) {
                    continue;
                }
                seen += 1;
                let in_root = roots.iter().any(|root| {
                    schedule
                        .graph()
                        .systems_in_set(*root)
                        .is_ok_and(|set| set.contains(&key))
                });
                if !in_root {
                    outside.push(format!("{name} in {label:?}"));
                }
            }
        }
    }
    assert!(
        seen > 0,
        "no `{CRATE}` systems seen: is Bevy's `debug` feature on?"
    );
    assert!(
        outside.is_empty(),
        "systems outside ElementsSystems: {outside:#?}"
    );
}

fn assert_settings<T: GetTypeRegistration>(registry: &TypeRegistry) {
    let name = std::any::type_name::<T>();
    let registration = registry
        .get(std::any::TypeId::of::<T>())
        .unwrap_or_else(|| panic!("{name} is not registered by ElementsPlugins"));
    assert!(
        registration.data::<ReflectResource>().is_some(),
        "{name} has no ReflectResource"
    );
    assert!(
        registration.data::<ReflectDefault>().is_some(),
        "{name} has no ReflectDefault"
    );
    #[cfg(feature = "serde")]
    {
        assert!(
            registration
                .data::<bevy::reflect::prelude::ReflectSerialize>()
                .is_some(),
            "{name} has no ReflectSerialize"
        );
        assert!(
            registration
                .data::<bevy::reflect::prelude::ReflectDeserialize>()
                .is_some(),
            "{name} has no ReflectDeserialize"
        );
    }
}

fn assert_component<T: GetTypeRegistration>(registry: &TypeRegistry) {
    let name = std::any::type_name::<T>();
    let registration = registry
        .get(std::any::TypeId::of::<T>())
        .unwrap_or_else(|| panic!("{name} is not registered by ElementsPlugins"));
    assert!(
        registration.data::<ReflectComponent>().is_some(),
        "{name} has no ReflectComponent"
    );
}

/// Every settings resource is reflected, defaulted, serializable and registered.
#[test]
fn settings_are_reflected() {
    let app = app();
    let registry = app.world().resource::<AppTypeRegistry>().read();
    assert_settings::<CelestialSettings>(&registry);
    assert_settings::<Climate>(&registry);
    assert_settings::<DustSettings>(&registry);
    assert_settings::<ErosionSettings>(&registry);
    assert_settings::<GeoLocation>(&registry);
    assert_settings::<HydrologySettings>(&registry);
    assert_settings::<RainbowSettings>(&registry);
    assert_settings::<RockSettings>(&registry);
    assert_settings::<SnowSettings>(&registry);
    assert_settings::<TrailSettings>(&registry);
    assert_settings::<WaterSettings>(&registry);
    assert_settings::<WaveSettings>(&registry);
    assert_settings::<WeatherOverrideSettings>(&registry);
    assert_settings::<WetnessSettings>(&registry);
    assert_settings::<WorldClock>(&registry);
}

/// Every public component is reflected and registered.
#[test]
fn components_are_reflected() {
    let app = app();
    let registry = app.world().resource::<AppTypeRegistry>().read();
    assert_component::<ElementsCamera>(&registry);
    assert_component::<ElementsFocus>(&registry);
    assert_component::<GroundFog>(&registry);
    assert_component::<Imprint>(&registry);
    assert_component::<Moon>(&registry);
    assert_component::<PrecipitationBlocker>(&registry);
    assert_component::<ScatteredRock>(&registry);
    assert_component::<Sun>(&registry);
    assert_component::<Terrain>(&registry);
    assert_component::<TerrainTextures>(&registry);
    assert_component::<WaterContact>(&registry);
    assert_component::<WaterPatch>(&registry);
    assert_component::<WaterSink>(&registry);
    assert_component::<WaterSource>(&registry);
    #[cfg(feature = "isomesh")]
    {
        assert_component::<VoxelGround>(&registry);
        assert_component::<VoxelSnow>(&registry);
    }
}
