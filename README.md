# Spider game

I don't know what to add here so I added a bunch of bs.

Project tree:
---

- **Entities, Components, States, Plugins:** Should be in the src folder because it's used in global context.
- **Systems:** Should be in a sub folder of plugins folder and being added by the plugins it self, the sub folder name should be the same name as the plugin.
- **Resources:** Should belong in the same file of the system that use it, if it is used by multiple systems then it should be in the plugin file.

## Quick commands:

### [Run]

```sh
cargo run
```

### [Run release]

```sh
cargo run --release
```

## Examples:

### [Bevy]

- https://github.com/rust-adventure/bevy-examples/tree/main/examples
- https://github.com/bevyengine/bevy/tree/main/examples

### [Ldtk]

- https://github.com/Trouv/bevy_ecs_ldtk/tree/main/examples
