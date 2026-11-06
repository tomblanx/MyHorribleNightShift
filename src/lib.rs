use crate::setup::setup;
use bevy::prelude::*;

mod setup;

pub fn app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_systems(Startup, setup);

    app
}
