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

## Hook points

You must call `registerPlugin` function for each script.
Method signature of the `registerPlugin` is the following:

## Functions

There are some functions to implement your own great script.

### ``

