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

Details of `config_schema`:

The config_schema parameter in the registerPlugin function is an array of objects that define the configuration options for your plugin. Each object in the array represents a single configuration option and specifies its properties and default values.

**Structure**

Each object within the config_schema array should have the following properties:

* `name` (String): The unique identifier for the configuration option.
* `type` (String): The data type of the configuration option (string, integer or hotkey).
* `default` (String or Number): The default value of the configuration option.
* `description` (String): A brief description of what the configuration option does or represents.

Hotkey type is emacs like notation. e.g. `C-t`

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

TBD about constants

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

### `kCGEventKeyDown`

- **Description**: Represents a key-down event in the system. This constant is used to identify events where a key is pressed.

### `kCGEventKeyUp`

- **Description**: Represents a key-up event. Use this constant to identify events where a key is released.

### `kCGEventFlagsChanged`

- **Description**: Indicates an event where the modifier flags (like Shift, Control, etc.) have changed. This is crucial for detecting state changes in modifier keys.

### `kCGKeyboardEventKeycode`

- **Description**: Used to access the keycode field in a keyboard event. This constant allows you to determine which key was pressed or released.

### `kCGEventFlagMaskNonCoalesced`

- **Description**: This constant is used to identify non-coalesced events. Non-coalesced events are those that report individual movements rather than the final position in a series of movements, providing finer granularity in event handling.
