use bevy::prelude::*;

#[cfg(feature = "dev")]
mod dev_tools;
fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);

    app.run()
}
