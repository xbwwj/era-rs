# era-rs

A Rust rewrite of [era][js]. This project is intended as a playground to
experiment with [Bevy][bevy]-based TUIs.

[js]: https://github.com/kyoheiu/era
[bevy]: https://bevy.org/

## TODO

- [ ] Refactor with bevy
  - [ ] first `bevy-ratatui`
  - [ ] then write a custom library from scratch with better bevy integration
- [ ] Survey and review other GUI/TUI clock UI design and functionality.
- [ ] Be more feature-rich
  - [ ] count down
  - [ ] Send notification
  - [ ] Plugin system
    - [ ] How plugin interact with UI? Bevy `RenderLayer`?
    - [ ] How to expose FFI?
    - [ ] Boa-engine
