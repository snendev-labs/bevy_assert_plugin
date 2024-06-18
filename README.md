# bevy_assert_plugins

A small extension trait for chainable method calls on `App` that check for dependency plugins and provide fallbacks as desired.

```rust
let mut app = App:;new();
app.assert_plugin::<MyPlugin>();
app.assert_plugin_else::<MyPlugin>(|| MyPlugin { ... });
app.assert_plugin_else_default::<MyPlugin>();
```
