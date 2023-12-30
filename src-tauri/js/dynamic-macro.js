let latest_flags = undefined;
const buffer = [];

register_plugin(
    "com.github.tokuhirom.onemoretime.dynamicmacro",
    "One more time",
    function (event, config) {
        if (event.type === "flags_changed") {
            latest_flags = event.flags;
        } else if (event.type === "keydown") {
            // TODO: call matches_hotkey_string
            buffer.unshift([event, config]);
            if (buffer.length > 6) {
                buffer.pop();
            }
        }
        console.log(`event detected :::${JSON.stringify(event)}, ${latest_flags}`);

        return true; // send event to the normal destination
    },
    [
        {
            "name": "shortcut",
            "type": "shortcut",
            "default": "C-t"
        }
    ]
);
