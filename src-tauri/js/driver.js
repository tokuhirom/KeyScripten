/** This is a driver of the MaguroMate */

let $$IDS = [];
let $$CALLBACKS = {};
let $$CONFIG_SCHEMAS = {};

// public API
function registerPlugin(id, name, callback, config_schema) {
    $$IDS.push(id); // to be unique?
    $$CALLBACKS[id] = callback;
    $$CONFIG_SCHEMAS[id] = config_schema;
}

function $$invokeEvent(event) {
    for (let i = 0; i < $$IDS.length; i++) {
        let id = $$IDS[i];
        let callback = $$CALLBACKS[id];
        let config_schema = $$CONFIG_SCHEMAS[id];
        const result = callback(event);
        if (!result) {
            return result;
        }
    }
    return true;
}
