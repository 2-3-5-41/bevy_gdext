# bevy_gdext

This library acts as the bridge between Bevy and Godot 4 using [Gdext](https://github.com/godot-rust/gdext). `bevy_gdext` provides traits, components, bundles, and resources that can be used inside bevy as raw bevy code, and translate automagicly over to godot.

## Work In Progress

Do note; this crate is under very early development, there will be a *LOT* of missing components, and features that we would all need in order to have a functioning Bevy x Godot4 game/app.

The API will change a lot, but the idea is to keep it simple to use, and automate everything in the background so you only have to use bevy knowledge, and zero godot knowledge.

## Side note

This crate was originally created to be the foundation of a current XR project that I'm working on; please be patient as I will unfortunately prioritize the features that I need from a crate like this.

## Contribution

All I ask; if you are wanting to help get all the 'base' features in place, and then some, try to fall in line with regular rust conventions, and mimic the rest of the API's namings and what nots. As this crate grows, contribution guidelines will change to be more scrutinizing for the sake of consistency and platform compatibility.

---

## Getting started

Make sure you have these dependencies in your `Cargo.toml`

```toml

[lib]
crate-type = ["cdylib"] # As per the `godot-rust/gdext` setup.

[dependencies]
bevy_gdext = { git = "https://github.com/2-3-5-41/bevy_gdext.git" }
bevy = { version = "0.10.0", default-features = false } # Disable `default-features` since Godot will be doing all the rendering.
godot = { git = "https://github.com/godot-rust/gdext.git" }

```
