# How to write your own script

## What's this document?

This document describes how to write your own script on **KeyScripten**.

### SYNOPSIS

Here's the complete example of the script.

```javascript
(function () {
    const id = "com.example.my.own.script";
    let latestFlags = undefined;

    registerPlugin(
        id,
        "My own script",
        "This is just an example.",
        function (event, config) {
            if (event.type === "flagsChanged") {
                console.log(`[${id}] flagsChanged: ${event.flags}`);
                latestFlags = event.flags;
            } else if (event.type === "keyDown") {
                console.log(`[${id}] keyDown: keycode=${event.keycode} flags=${latestFlags}`);
                if (config.hotkey.matches(latestFlags, event.keycode)) {
                    console.log(`[${id}] Handled hotkey`);
                    return false;
                }
            }
            return true; /* true means, KeyScripten should send the keycode to the application. */
        },
        [ /* configuration parameters */
            {
                "name": "hotkey",
                "type": "hotkey",
                "default": "S-M-C-t",
                "description": "Key sequence for something.",
            },
            {
                "name": "size",
                    "type": "integer",
                    "description": "Size of something.",
                    "default": "64"
            }
        ]
    )
})();
```    

## Register your own script

You must call `registerPlugin` function for each script.
Method signature of the `registerPlugin` is the following:

```javascript
registerPlugin(id, name, description, callback, config_schema);
```

Parameters
* `id` (String): A unique identifier for the plugin. It must be unique across all registered plugins.
* `name` (String): A human-readable name for the plugin.
* `description` (String): A short description of what the plugin does.
* `callback` (Function): The function that will be called when the plugin is executed. The function receives event and config parameters, where event contains details about the current event and config contains the plugin's configuration.
* `config_schema` (Array): An array of configuration parameters that define the structure of the plugin's configuration. Each parameter in the array should be an object specifying the name, type, default value, and description of a configuration parameter.

### Details of `callback`:

```javascript
function callback(event, config) {
    // Plugin logic goes here
}
```

#### Parameters

* `event` (Object): An object representing the event that triggered the callback. This object contains details about the event, such as its type and any relevant data associated with it.
* `config` (Object): An object containing the current configuration settings for the plugin, as defined by the config_schema. This allows the callback function to access and react to user-defined settings.

#### Functionality

The callback function is invoked whenever a relevant event occurs. The specific events that trigger the callback depend on how the **KeyScripten** system is designed and what events it supports. The function must process the `event` object, perform the necessary actions based on the event type and data, and respect the configurations specified in `config`.

#### Return value

The callback function should return a boolean value:

* `true`: Indicates that the event was not handled by the plugin and should be processed further or passed to other plugins or the system.
* `false`: Indicates that the event was fully handled and consumed by the plugin, and no further processing is needed for this event.

### Details of `config_schema`:

The config_schema parameter in the registerPlugin function is an array of objects that define the configuration options for your plugin. Each object in the array represents a single configuration option and specifies its properties and default values.

**Structure**

Each object within the config_schema array should have the following properties:

* `name` (String): The unique identifier for the configuration option.
* `type` (String): The data type of the configuration option (string, integer or hotkey).
* `default` (String or Number): The default value of the configuration option.
* `description` (String): A brief description of what the configuration option does or represents.

Hotkey type is emacs like notation. e.g. `C-t`

## Objects

### The `event` object

This is passed to the callback function.
The `event` object has following fields:


* `type` (String): The field indicates the event type. One of the `flagsChanged`, `keyUp` or `keyDown`.
* `flags` (Number): Bit field of the flags. It's based on `CGEventFlags`. See `Constants` section.
* `keycode` (Number): KeyCode.

### The `hotkey` object

If the configuration field is typed as a `hotkey`, it would be a `hotkey` object.
`hotkey` object has following methods:

#### `hotkey.matches(flags, keycode)`

Parameters are `flags` and `keycode`.
`flags` contains the information about pressing modifier keys. it's taken from the `flagsChanged` event.

Return value is boolean, it returns true i hotkey matches the flags and keycode.

## Functions

There are some functions to implement your own great script.

### `sendFlagsChangedEvent`

#### Purpose

Sends an event indicating that the flags (such as modifier keys) have changed.

#### Usage

```javascript
sendFlagsChangedEvent(flags);
```

#### Parameters

* `flags` (Integer): An integer representing the current state of the flags. This is typically a bitmask of the flags.

Available for bit flags: `kCGEventFlagMaskAlphaShift`, `kCGEventFlagMaskShift`, `kCGEventFlagMaskControl`, `kCGEventFlagMaskAlternate`, `kCGEventFlagMaskCommand`, `kCGEventFlagMaskHelp`, `kCGEventFlagMaskSecondaryFn`, `kCGEventFlagMaskNumericPad`, `kCGEventFlagMaskNonCoalesced`

#### Return Value

Returns `undefined`. In case of an error, it throws a TypeError with a message detailing the issue.

#### Example

```javascript
sendFlagsChangedEvent(0x101); // Example bitmask
```

### `sendKeyboardEvent`

#### Purpose

Sends a keyboard event, such as a key press or release.

#### Usage

```javascript
sendKeyboardEvent(keycode, flags, pressed);
```

#### Parameters

* `keycode` (Integer): The keycode of the key being pressed or released.
* `flags` (Integer): An integer bitmask representing the state of modifier flags during the event.
* `pressed` (Boolean): A boolean indicating whether the key is being pressed (true) or released (false).

#### Return Value

Returns `undefined`. In case of an error, it throws a TypeError with a message detailing the issue.

#### Example

```javascript
sendKeyboardEvent(13, 0x101, true); // Keycode 13 (Enter key), with flags, being pressed
```

Certainly, documenting constants is essential for users to understand and effectively use them in their scripts. Below is a documentation section for the constants registered in your code:

## Constants

In **KeyScripten**, several constants are registered for use in scripting. These constants typically represent specific values related to keyboard events and their properties. Understanding these constants is crucial for handling keyboard events correctly.

### Key event constants

Maybe, there's no use cases.

#### `kCGEventKeyDown`

- **Description**: Represents a key-down event in the system. This constant is used to identify events where a key is pressed.

#### `kCGEventKeyUp`

- **Description**: Represents a key-up event. Use this constant to identify events where a key is released.

#### `kCGEventFlagsChanged`

- **Description**: Indicates an event where the modifier flags (like Shift, Control, etc.) have changed. This is crucial for detecting state changes in modifier keys.

#### `kCGKeyboardEventKeycode`

- **Description**: Used to access the keycode field in a keyboard event. This constant allows you to determine which key was pressed or released.

#### `kCGEventFlagMaskNonCoalesced`

- **Description**: This constant is used to identify non-coalesced events. Non-coalesced events are those that report individual movements rather than the final position in a series of movements, providing finer granularity in event handling.

### `CGEventFlags` - Keyboard Event Flag Constants 

Absolutely, I'll document these constants in the same format as the previous ones, assuming they are registered using a similar method in your **KeyScripten** environment.

In **KeyScripten**, several constants related to keyboard event flags are registered for use in scripting. These constants represent specific modifier keys and other keyboard-related states.

#### `kCGEventFlagMaskAlphaShift`

- **Description**: Represents the alpha shift (Caps Lock) modifier key. This constant is used to identify events where the Caps Lock key's state is involved.

#### `kCGEventFlagMaskShift`

- **Description**: Represents the Shift key modifier. Use this constant to detect if the Shift key was pressed during an event.

#### `kCGEventFlagMaskControl`

- **Description**: Identifies the Control key modifier. This constant is crucial for detecting events where the Control key is involved.

#### `kCGEventFlagMaskAlternate`

- **Description**: Represents the Alternate (or Option) key modifier. Use this to detect events involving the Alternate key.

#### `kCGEventFlagMaskCommand`

- **Description**: Indicates the Command key modifier. This constant is used for identifying events where the Command key is used.

#### `kCGEventFlagMaskHelp`

- **Description**: Represents the Help key modifier. Use this constant to identify events involving the Help key.

#### `kCGEventFlagMaskSecondaryFn`

- **Description**: Indicates the secondary function (Fn) key modifier. This is used for events where the

Fn key plays a role, particularly on keyboards where the Fn key modifies the behavior of other keys.

#### `kCGEventFlagMaskNumericPad`

- **Description**: Represents events involving keys on the numeric pad. This constant is useful for differentiating between numeric pad input and other keyboard inputs.

#### `kCGEventFlagMaskNonCoalesced`

- **Description**: This constant is used to identify non-coalesced events in the context of keyboard inputs. Non-coalesced events provide more detailed and individual reporting of input events, offering finer control in handling these inputs.

## `Key` object

The `Key` object contains the keycode map.

```json
{
    "ALT": 58,
    "ALT_GR": 61,
    "BACKSPACE": 51,
    "CAPS_LOCK": 57,
    "CONTROL_LEFT": 59,
    "CONTROL_RIGHT": 62,
    "DOWN_ARROW": 125,
    "ESCAPE": 53,
    "F1": 122,
    "F10": 109,
    "F11": 103,
    "F12": 111,
    "F2": 120,
    "F3": 99,
    "F4": 118,
    "F5": 96,
    "F6": 97,
    "F7": 98,
    "F8": 100,
    "F9": 101,
    "FUNCTION": 63,
    "LEFT_ARROW": 123,
    "META_LEFT": 55,
    "META_RIGHT": 54,
    "RETURN": 36,
    "RIGHT_ARROW": 124,
    "SHIFT_LEFT": 56,
    "SHIFT_RIGHT": 60,
    "SPACE": 49,
    "TAB": 48,
    "UP_ARROW": 126,
    "BACK_QUOTE": 50,
    "NUM1": 18,
    "NUM2": 19,
    "NUM3": 20,
    "NUM4": 21,
    "NUM5": 23,
    "NUM6": 22,
    "NUM7": 26,
    "NUM8": 28,
    "NUM9": 25,
    "NUM0": 29,
    "MINUS": 27,
    "EQUAL": 24,
    "Q": 12,
    "W": 13,
    "E": 14,
    "R": 15,
    "T": 17,
    "Y": 16,
    "U": 32,
    "I": 34,
    "O": 31,
    "P": 35,
    "LEFT_BRACKET": 33,
    "RIGHT_BRACKET": 30,
    "A": 0,
    "S": 1,
    "D": 2,
    "F": 3,
    "G": 5,
    "H": 4,
    "J": 38,
    "K": 40,
    "L": 37,
    "SEMI_COLON": 41,
    "QUOTE": 39,
    "BACK_SLASH": 42,
    "Z": 6,
    "X": 7,
    "C": 8,
    "V": 9,
    "B": 11,
    "N": 45,
    "M": 46,
    "COMMA": 43,
    "DOT": 47,
    "SLASH": 44
}
```
