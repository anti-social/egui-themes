# egui-themes
An egui widget that provides super-simple color theming.

## Links

* Crates.io: https://crates.io/crates/egui-themes
* docs.rs: https://docs.rs/egui-themes
* Github: https://github.com/Resonanz/egui-themes

## What is egui-themes?

egui-themes is a crate that provides super-simple color theming capability for egui projects.

The current theme colors are taken from Catppuccin (https://crates.io/keywords/catppuccin).

Additional color themes may be added. Please submit your own and I may incorporate them into this crate.

Please submit an issue on Github if you have suggestions or improvements.

## Usage

In ```Cargo.toml``` add the following dependency:

```
[dependencies]
egui-themes = 0.1.0  <--- The latest version number can be found on Crates.io.
```

Or you could use the following if developing locally:
```
[dependencies]
egui-themes = { path = "/Github/egui-themes/" }
```

### The following asumes you are using eframe_template:

In ```app.rs``` import the crate using:

```use egui_themes::{StateMachine, MOCHA};```

Using the ```TemplateApp``` struct, define a ```run_once``` boolean and a ```StateMachine``` variable to hold the current theme:

```
pub struct TemplateApp {
    run_once: bool,
    my_theme: StateMachine,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            run_once: false,
            my_theme: egui_themes::StateMachine::new(),
        }
    }
}
```

Inside ```fn update...``` set the startup theme state using the ```run_once``` boolean:

```
fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
    if self.run_once == false {
        self.run_once = true;
        self.my_theme.set_theme(ctx, &MOCHA);
    }
```
 Then in the main loop:

```
// Theme cycle button
let b = ui.add(egui::Button::new("☀🌙").sense(Sense::click()));

if b.clicked() {
    self.my_theme.rotate_theme(&ctx);
} else if b.hovered() {
    b.on_hover_text("Click for next theme...");
}
```




## Video
https://github.com/user-attachments/assets/ddcfce39-8377-440f-bce6-b98e7945c441
