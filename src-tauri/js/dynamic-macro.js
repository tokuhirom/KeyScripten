let latest_flags = undefined;
const buffer = [];

function run_dynamic_macro() {
    send_flags_changed_event(kCGEventFlagMaskNonCoalesced);

    const KEY_A = 0; // TODO: expose this style keycodes from rust world
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
            // TODO config.hotkey.matches(latest_flags, keycode)
            if (matches_hotkey_string(latest_flags, event.keycode, "C-t")) {
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
            "name": "hotkey",
            "type": "hotkey",
            "default": "C-t"
        }
    ]
);
