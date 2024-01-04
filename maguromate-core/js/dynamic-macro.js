let latest_flags = undefined;
const buffer = [];

/**
 * Run dynamic macro.
 * @returns {boolean} Return true if sent keyboard event, false otherwise.
 */
function run_dynamic_macro() {
    const repeatSize = checkRepeat(buffer);

    if (repeatSize !== null) {
        console.log(`DynamicMacro: repeat detected`)
        sendFlagsChangedEvent(kCGEventFlagMaskNonCoalesced);

        const front = buffer.slice(0, repeatSize);
        for (const keyState of front.reverse()) {
            sendKeyboardEvent(keyState[0], keyState[1], true);
        }

        sendFlagsChangedEvent(latest_flags);

        return true;
    } else {
        const patternXYX = checkPatternXYX(buffer);
        if (patternXYX) {
            console.log(`DynamicMacro: Predicted`)
            sendFlagsChangedEvent(kCGEventFlagMaskNonCoalesced);

            for (const keyState of patternXYX.Y.reverse()) {
                sendKeyboardEvent(keyState[0], keyState[1], true);
                buffer.unshift(keyState);
            }

            sendFlagsChangedEvent(latest_flags);

            return true;
        } else {
            console.warn("No pattern found: " + JSON.stringify(buffer));
            return false;
        }
    }
}

function checkPatternXYX(buffer) {
    let longestX = null;
    let shortestY = null;

    for (let xLen = 1; xLen <= buffer.length / 2; xLen++) {
        for (let yStart = xLen; yStart + xLen <= buffer.length; yStart++) {
            let X1 = buffer.slice(0, xLen);
            let Y = buffer.slice(xLen, yStart);
            let X2 = buffer.slice(yStart, yStart + xLen);

            if (JSON.stringify(X1) === JSON.stringify(X2)) {
                if (!longestX || X1.length > longestX.length || (X1.length === longestX.length && Y.length < shortestY.length)) {
                    longestX = X1;
                    shortestY = Y;
                }
            }
        }
    }

    return longestX && shortestY ? { X: longestX, Y: shortestY } : null;
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
    "com.github.tokuhirom.codekeys.dynamicmacro",
    "Dynamic Macro",
    `This plugin provides a dynamic macro generation feature, learning from the user's repetitive
    actions and automatically defining and executing them as macros. After a user repeatedly
    performs certain key operations, pressing a designated hotkey triggers the plugin to
    recognize these actions and generate a macro for automatic re-execution. This functionality
    streamlines repetitive tasks and enhances productivity. Based on the dynamic macro generation
    technology developed by Mr. Toshiyuki Masui.
    
    Ref. http://www.pitecan.com/papers/JSSSTDmacro/JSSSTDmacro.html
    `,
    function (event, config) {
        // console.log(`config=${JSON.stringify(config)}`);

        if (event.type === "flagsChanged") {
            latest_flags = event.flags;
        } else if (event.type === "keyDown") {
            if (config.hotkey.matches(latest_flags, event.keycode)) {
                return !run_dynamic_macro();
            }

            buffer.unshift([event.keycode, latest_flags]);
            if (buffer.length > 10) {
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
            "default": "C-t",
            "description": "Repeat key",
        },
        {
            "name": "buffer_size",
            "type": "integer",
            "description": "Maximum history size. This plugin consumes O(N**2) for each typing. Do not set too large buffer.",
            "default": "64"
        }
    ]
);
