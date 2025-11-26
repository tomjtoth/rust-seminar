# Egui version

A very exhaustive online demo can be found [here](https://www.egui.rs/), taking ~ 3..20ms/frame to render

## likes

- simple setup for the desktop
  - simply run `cargo add eframe`
  - [reference](https://docs.rs/eframe/latest/eframe/#usage-native)
  - OS deps were already satisfied due to Dioxus setup (?)
- no deviation from plain rust, simple function calls with same params
- reactive by default, continuous if specified

## dislikes

- prop drilling
  - passing `&mut state` down the tree
  - could probably be mitigated with LazyCell
    - 1 failed attempt
- unfamiliar and fractured state management
  - allocating in [app.rs](./src/app.rs#L30-41), resetting in [navbar.rs](./src/components/navbar.rs)
- unfamiliar/impossible(?) styling
  - 2 adjacent blocks centered horizontally
  - colored buttons? [WiP](https://github.com/emilk/egui/issues/3284)
- callback more complex due to mutation of state
- setting up for web is complex
  - but works out of the box
  - [reference](https://github.com/emilk/eframe_template)
- setting up for android
  - [reference](https://github.com/fredrik-hammar/egui-android-demo)
  - failed to custom-tailor to this demo

## Running on the desktop (ArchLinux)

```sh
cargo run
```

## Running on the Web

- [Install trunk](https://github.com/emilk/eframe_template?tab=readme-ov-file#web-locally)

  ```sh
  cargo install --locked trunk
  ```

- then serve the app (at http://localhost:8081)

  ```sh
  trunk serve
  ```
