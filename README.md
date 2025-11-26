# GUI on Rust

Looking for a GUI framework with the below personal critera on [Are We GUI Yet?](https://areweguiyet.com/)

- must have a promising-looking download count
- must be written in pure Rust (`*.rs` files only)
- must be cross-platform, at least compile to
  - the Linux desktop
  - the browser
  - Android

## Total downloads as of 26.11

|                               package | downloads | description                                                                                                 |
| ------------------------------------: | :-------: | ----------------------------------------------------------------------------------------------------------- |
|                       core-foundation |   174M    | MacOS-only                                                                                                  |
|                               windows |   153M    | Windows-only                                                                                                |
|                               winsafe |    21M    | Windows-only                                                                                                |
|           [**egui**](https://egui.rs) |    11M    | Native alternative to _imgui_ (immediate) ([Android](https://github.com/fredrik-hammar/egui-android-demo))  |
|                                 tauri |   8.4M    | [not standalone](https://v2.tauri.app/start/)                                                               |
|                                   yew |   3.5M    | [browser-only](https://yew.rs/docs/getting-started/build-a-sample-app)                                      |
|                                _gtk4_ |   2.2M    | [no Android support](https://github.com/gtk-rs/gtk4-rs/issues/1997)                                         |
|                                leptos |   1.9M    | [browser-only](https://book.leptos.dev/view/01_basic_component.html)                                        |
|                                _fltk_ |   1.3M    | Rust bindings for fltk                                                                                      |
|                                _iced_ |   1.3M    | seemingly same [difficulties in styling](https://book.iced.rs/the-runtime.html#the-ice-wizard) as with egui |
|                              imgui-rs |   832k    | [Rust bindings to the C++ lib](https://github.com/ocornut/imgui) (immediate)                                |
| [**dioxus**](https://dioxuslabs.com/) |   841k    | React-clone                                                                                                 |
|                                 slint |   762k    | [components declared via DSL](https://docs.slint.dev/latest/docs/slint/tutorial/memory_tile/)               |
|                                  crux |   177k    | ignored (wait for more DLs)                                                                                 |

### [Immediate mode](<https://en.wikipedia.org/wiki/Immediate_mode_(computer_graphics)>)

Quite "new" design approach, surfaced around ~2000

- upon client calls UI elements are rendered directly and reconstructed from ground up on

  - every single frame (continuous mode = 60 / sec)
  - every single frame when there's user-interaction (reactive mode, egui defaults to this)

- state is not managed by the Graphics Library (GL)

  - maintain your state in app code
  - "components" exist within the rendered frame

    - handling user-interaction is _different_

  - can be very performant

- used for prototyping, editors and in-game debuggers/menus(?)

### [Retained mode](https://en.wikipedia.org/wiki/Retained_mode)

The more traditional design approach

- contents and behavior of UI components are declared by the client, created once and stored in a persistent abstract tree by GL
- state alongside the whole rendered scenery is retained between frames by GL
- GL decides what and when to change on scene and renders accordingly

## dioxus (vs next.js) vs egui

Compared the below 3 and implement basic components:

- `dioxus`: works in _retained-mode_ and uses an enum router
- `egui`: works in _immediate-mode_ and also uses an "enum router"
- `next.js` **as reference**: works in _retained-mode_ and uses App router

### Overall Layout

- [next](./next/app/layout.tsx#28-35) produces an HTML tree
- [dx](./dioxus/src/main.rs#21-32), too
- [egui](./egui/src/app.rs#L58-71) renders everything to a canvas

### NavBar

- [next](./next/app/_components/navbar.tsx)
- [dx](./dioxus/src/components/navbar.rs)
- [egui](./egui/src/components/navbar.rs) - resets state manually when changing view, to conform to Next

### Router

- [next 🌍](https://nextjs.org/docs/app/getting-started/project-structure) relies on your folder structure and naming conventions
- [dx](./dioxus/src/routes.rs)
- [egui](./egui/src/router.rs)

### Counters

- [next](./next/app/counters/page.tsx)
- [dx](./dioxus/src/components/counters/mod.rs)
- [egui](./egui/src/components/counters/mod.rs) - resetting state manually in navbar

### Controlled input

- [next](./next/app/controlled-input/page.tsx)
- [dx](./dioxus/src/components/controlled_input.rs)
- [egui](./egui/src/components/controlled_input.rs) - resets state in navbar

### Context providers

- [next](./next/app/context-providers/page.tsx)
- [dx](./dioxus/src/components/context_provider/mod.rs)
- [egui](./egui/src/components/context_provider/mod.rs) - resetting state manually in navbar

### Global state

- [dx](./dioxus/src/components/global_state/mod.rs)
- [next](./next/app/global-state/page.tsx)
- [egui](./egui/src/components/global_counters/mod.rs) - no manual reset of state

### Callbacks

- [dx](./dioxus/src/components/callback/mod.rs)
- [next](./next/app/callbacks/page.tsx)
- [egui](./egui/src/components/callback/mod.rs) - resetting state manually in navbar

### Fullstack

- [next](./next/app/fullstack/page.tsx)
- [dx](./dioxus/src/components/fullstack/mod.rs)
- egui 🤷‍♂️

## Bonus

Misc. remarks, setup steps on both [Dioxus](./dioxus/README.md) and [egui](./egui/README.md) (and [Next](./next/README.md)).
And my halfway-migrated (then scrapped) [GitHub Pages](https://github.com/tomjtoth/tomjtoth.github.io/tree/dioxus) from January.

## Thanks for reading

<span style="font-size: 50px;">🙏🤓</span>
