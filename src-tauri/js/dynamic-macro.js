let latest_flags = undefined;
const buffer = [];

function run_dynamic_macro() {
    const size = checkRepeat(buffer);

    if (size !== null) {
        sendFlagsChangedEvent(kCGEventFlagMaskNonCoalesced);

        const front = buffer.slice(0, size);
        for (const keyState of front.reverse()) {
            sendKeyboardEvent(keyState[0], keyState[1], true);
        }

        sendFlagsChangedEvent(latest_flags);
    } else {
        console.warn("No repeats!!!: " + JSON.stringify(buffer));
    }
}

function checkRepeat(buffer) {
    for (let size = buffer.length / 2; size >= 1; size--) {
        let front = buffer.slice(0, size);
        let rear = buffer.slice(size, size * 2);
        // console.log("front=" + front + " rear=" + rear);
        if (JSON.stringify(front) === JSON.stringify(rear)) {
            return size;
        }
    }
    return null;
}

registerPlugin(
    "com.github.tokuhirom.onemoretime.dynamicmacro",
    "One more time",
    function (event, config) {
        // console.log(`config=${JSON.stringify(config)}`);

        if (event.type === "flagsChanged") {
            latest_flags = event.flags;
        } else if (event.type === "keyDown") {
            if (config.hotkey.matches(latest_flags, event.keycode)) {
                run_dynamic_macro();
                return false;
            }

            buffer.unshift([event.keycode, latest_flags]);
            if (buffer.length > 6) {
                buffer.pop();
            }
        }
        // console.log(`event detected :::${JSON.stringify(event)}, ${latest_flags}`);

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
