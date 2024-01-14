(function () {
    const id = /*ID*/{};
    let latestFlags = undefined;

    registerPlugin(
        id,
        /*NAME*/{},
        /*DESC*/{},
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
