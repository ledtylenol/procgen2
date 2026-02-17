use bevy::prelude::*;

#[cfg(feature = "dev")]
mod dev_tools;

mod voxel;
fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(AppPlugin);
    app.run()
}

struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}
