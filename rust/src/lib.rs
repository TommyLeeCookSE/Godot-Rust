use godot::prelude::*;

mod card;

struct Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Extension {}
