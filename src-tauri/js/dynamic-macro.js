let latest_flags = undefined;
const buffer = [];

function run_dynamic_macro() {
    console.log("run_dynamic_macro");
    send_flags_changed_event(0);

    const KEY_A = 0;
    send_keyboard_event(KEY_A, 0, true);

    send_flags_changed_event(latest_flags);
}

register_plugin(
    "com.github.tokuhirom.onemoretime.dynamicmacro",
    "One more time",
    function (event, config) {
        if (event.type === "flags_changed") {
            latest_flags = event.flags;
        } else if (event.type === "keydown") {
            if (matches_hotkey_string(latest_flags, event.keycode, "C-t")) {
                console.log("KKKKKKKKKKKKKKKKKKKKYAY!!! shortcut!!!!");
                run_dynamic_macro();
                return false;
            }
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
