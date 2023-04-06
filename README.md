# bevy_action

Simple action system for Bevy.

## Introduction

This plugin exists mainly to facilitate a common action system for other plugins to hook in to (e.g. [bevy_action_animation](https://github.com/undersquire/bevy_action_animation)).

It also serves as a way to decouple certain parts of game logic to avoid hard-coding things, such as hard-coding input handling to actions rather than directly to movement logic.

There are three main components, the `Action` trait, the `ActionEvent` event, and the `ActionPlugin`.

## Usage

Start by defining your own type to represent the actions (enum is recommended).
You will then need to derive the required traits and manually implement the `Action` trait (this will be derivable in the future):

```rs
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, TypeUuid)]
#[uuid = "UUID_HERE"]
enum MyAction {
  #[default]
  Idle,
  Left,
  Right,
  Jump
}

impl Action for MyAction {}
```

Next, register the plugin using this type as the generic parameter:

```rs
fn main() {
  App::new()
    ...
    .add_plugin(ActionPlugin::<MyAction>::default())
    ...
}
```

Now, `ActionEvent` can be used with `EventReader` and `EventWriter` just like any normal Bevy event.

## Compatibility

 **Bevy Version** | **Plugin Version**
---|---
0.10.x | 0.1
