/** This is a driver of the CodeKeys */

let $$IDS = [];
let $$NAMES = [];
let $$DESCRIPTIONS = [];
let $$CALLBACKS = {};
let $$CONFIG_SCHEMAS = {};
let $$CONFIG = {};
let app_config = JSON.parse(loadAppConfigJson());

// public API
function registerPlugin(id, name, description, callback, config_schema) {
    $$IDS.push(id); // to be unique?
    $$NAMES[id] = name;
    $$DESCRIPTIONS[id] = description;
    $$CALLBACKS[id] = callback;
    $$CONFIG_SCHEMAS[id] = config_schema;
    $$CONFIG[id] = $$build_config(id, config_schema);

    console.log(`Registered plugin: id=${id} name=${name} config=${JSON.stringify($$CONFIG[id])}`);
}

function reloadConfig() {
    for (const id of Object.keys($$CONFIG)) {
        $$CONFIG[id] = $$build_config(id, $$CONFIG_SCHEMAS[id]);
    }
}

function $$build_config(id, config_schema) {
    const config = {};
    for (const item of config_schema) {
        const value = (((app_config.plugins || {})[id] || {}).config || {})[item.name] || item.default;

        switch (item.type) {
            case "hotkey":
                console.log(`Parsing hotkey: ${id}: ${value}`)
                let hotkey = new HotKey(value);
                config[item.name] = hotkey;
                break;
            case "string":
                config[item.name] = value;
                break;
            case "integer":
                config[item.name] = parseInt(value, 10);
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
function $$invokeEvent(event, needsConfigReload) {
    if (needsConfigReload) {
        console.log("Reloading configuration file")
        reloadConfig();
    }

    for (let i = 0; i < $$IDS.length; i++) {
        let id = $$IDS[i];
        let callback = $$CALLBACKS[id];
        let config = $$CONFIG[id];

        try {
            const result = callback(event, config);
            if (!result) {
                return result;
            }
        } catch (e) {
            console.log(`Cannot invoke the ${id}: ${e}`);
            return true;
        }
    }
    return true;
}

function $$getConfigSchema(event) {
    const result = [];

    for (let i = 0; i < $$IDS.length; i++) {
        let id = $$IDS[i];
        let name = $$NAMES[id];
        let description = $$DESCRIPTIONS[id];
        let config_schema = $$CONFIG_SCHEMAS[id];

        result.push({
            "id": id,
            "name": name,
            "description": description,
            "config": config_schema,
        });
    }
    const json = JSON.stringify({
        "plugins": result
    });
    console.log(`json=${json}`);
    return json;
}

function $$unloadPlugin(plugin_id) {
    if (!$$IDS.includes(plugin_id)) {
        console.log(`Plugin with id=${plugin_id} is not registered.`);
        return;
    }

    const index = $$IDS.indexOf(plugin_id);
    if (index > -1) {
        $$IDS.splice(index, 1);
    }

    delete $$NAMES[plugin_id];
    delete $$DESCRIPTIONS[plugin_id];
    delete $$CALLBACKS[plugin_id];
    delete $$CONFIG_SCHEMAS[plugin_id];
    delete $$CONFIG[plugin_id];

    console.log(`Unloaded plugin: id=${plugin_id}`);
}
