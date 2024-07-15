use bevy::prelude::{App, Plugin};

/// This trait is an extension trait for bevy's App providing methods that make assertions on the
/// plugins attached in the app.
pub trait AssertPluginAppExt {
    /// This function checks whether `P` has been registered in the `App``.
    /// If so, it does nothing; otherwise, it panics.
    fn assert_plugin<P: Plugin>(&mut self) -> &mut Self;

    /// This function checks whether `P` has been registered in the `App``.
    /// If so, it does nothing; otherwise, it runs the provided constructor.
    fn assert_plugin_else<P: Plugin>(&mut self, make_plugin: impl FnOnce() -> P) -> &mut Self;

    /// This function checks whether `P` has been registered in the `App``.
    /// If so, it does nothing; otherwise, it adds the default.
    fn assert_plugin_else_default<P: Plugin + Default>(&mut self) -> &mut Self;
}

impl AssertPluginAppExt for App {
    /// This function checks whether `P` has been registered in the `App``.
    /// If so, it does nothing; otherwise, it panics.
    fn assert_plugin<P: Plugin>(&mut self) -> &mut Self {
        assert!(self.is_plugin_added::<P>());
        self
    }

    /// This function checks whether `P` has been registered in the `App``.
    /// If so, it does nothing; otherwise, it runs the provided constructor.
    fn assert_plugin_else<P: Plugin>(&mut self, make_plugin: impl FnOnce() -> P) -> &mut Self {
        if !self.is_plugin_added::<P>() {
            self.add_plugins(make_plugin());
        }
        self
    }

    /// This function checks whether `P` has been registered in the `App``.
    /// If so, it does nothing; otherwise, it adds the default.
    fn assert_plugin_else_default<P: Plugin + Default>(&mut self) -> &mut Self {
        if !self.is_plugin_added::<P>() {
            self.add_plugins(P::default());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use bevy::{
        core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
        pbr::PbrPlugin,
        time::TimePlugin,
        MinimalPlugins,
    };

    use super::*;

    #[test]
    fn test_minimal_plugins() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.assert_plugin::<TaskPoolPlugin>();
        app.assert_plugin::<TypeRegistrationPlugin>();
        app.assert_plugin::<FrameCountPlugin>();
        app.assert_plugin::<TimePlugin>();
    }

    #[test]
    #[should_panic]
    fn test_assert_panics() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.assert_plugin::<PbrPlugin>();
    }

    #[derive(Default)]
    struct MyPlugin;

    impl Plugin for MyPlugin {
        fn build(&self, _app: &mut App) {}
    }

    #[test]
    fn test_assert_else_fallback() {
        let mut app = App::new();
        app.assert_plugin_else::<MyPlugin>(|| MyPlugin);
        app.assert_plugin::<MyPlugin>();
    }

    #[test]
    fn test_assert_else_default() {
        let mut app = App::new();
        app.assert_plugin_else_default::<MyPlugin>();
        app.assert_plugin::<MyPlugin>();
    }
}
