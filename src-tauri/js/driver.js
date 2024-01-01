/** This is a driver of the MaguroMate */

let $$IDS = [];
let $$CALLBACKS = {};
let $$CONFIG_SCHEMAS = {};
let $$CONFIG = {};
let app_config = JSON.parse(loadAppConfigJson());

// public API
function registerPlugin(id, name, callback, config_schema) {
    $$IDS.push(id); // to be unique?
    $$CALLBACKS[id] = callback;
    $$CONFIG_SCHEMAS[id] = config_schema;

    const config = $$build_config(id, config_schema);
    $$CONFIG[id] = config;
    console.log(`Registered plugin: id=${id} config=${JSON.stringify(config)}`);
}

function $$build_config(id, config_schema) {
    const config = {};
    for (const item of config_schema) {
        const value = ((app_config.plugins || {})[id] || {})[item.name] || item.default;

        switch (item.type) {
            case "hotkey":
                console.log(`Parsing hotkey: ${id}: ${value}`)
                let hotkey = new HotKey(value);
                config[item.name] = hotkey;
                break;
            case "string":
                config[item.name] = value;
                break;
            default:
                throw new Error(`Unknown type for plugin '${id}'(${item.name}): '${item.type}'`)
        }
        console.log(JSON.stringify([item, value]));
    }
    console.log(JSON.stringify(config));
    return config;
}

// called by js.rs
function $$invokeEvent(event) {
    for (let i = 0; i < $$IDS.length; i++) {
        let id = $$IDS[i];
        let callback = $$CALLBACKS[id];
        let config = $$CONFIG[id];
        const result = callback(event, config);
        if (!result) {
            return result;
        }
    }
    return true;
}
