
# Runic

A 2D graphics library and windowing interface for Rust with a focus on making GUIs.

The idea isn't so much to provide a widget toolkit, but rather the idea of Runic is that your app has one central data structure, which can be drawn to the screen, and then the user directly manipulates the data with mouse/keyboard/etc. For example, a text editor or a graph editor. As a result, Runic is a lot lower-level than a toolkit like Druid.

## Platform Support

| Platform | Support |
| -------- | ------- |
| Windows | ✓ |
| Linux (Wayland) | ✓ (Window chrome is provided by winit, requires Cairo with Cairo GL support) |
| Linux (Xorg) | 🗙 (Code is there, just needs updated) |
| MacOS | 🗙 (Again, code is there, but MacOS has changed to break it) |

