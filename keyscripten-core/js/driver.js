/** This is a driver of the KeyScripten */

const $$IDS = [];
const $$NAMES = [];
const $$DESCRIPTIONS = [];
const $$CALLBACKS = {};
const $$CONFIG_SCHEMAS = {};
const $$CONFIG = {};
const app_config = JSON.parse($$loadAppConfigJson());

// public API
function registerPlugin(id, name, description, callback, config_schema) {
	if (!$$IDS.includes(id)) {
		$$IDS.push(id);
	}

	$$NAMES[id] = name;
	$$DESCRIPTIONS[id] = description;
	$$CALLBACKS[id] = callback;
	$$CONFIG_SCHEMAS[id] = config_schema;
	$$CONFIG[id] = buildConfig(id, config_schema);

	console.log(
		`Registered plugin: id=${id} name=${name} config=${JSON.stringify($$CONFIG[id])}`,
	);
}

const reloadConfig = () => {
	for (const id of Object.keys($$CONFIG)) {
		$$CONFIG[id] = buildConfig(id, $$CONFIG_SCHEMAS[id]);
	}
};

const buildConfig = (id, config_schema) => {
	const config = {};
	for (const item of config_schema) {
		const value =
			// biome-ignore lint/complexity/useOptionalChain: <explanation>
			(((app_config.plugins || {})[id] || {}).config || {})[item.name] ||
			item.default;

		switch (item.type) {
			case "hotkey": {
				console.log(`Parsing hotkey: ${id}: ${value}`);
				const hotkey = new HotKey(value);
				config[item.name] = hotkey;
				break;
			}
			case "string":
				config[item.name] = value;
				break;
			case "integer":
				config[item.name] = Number.parseInt(value, 10);
				break;
			default:
				throw new Error(
					`Unknown type for plugin '${id}'(${item.name}): '${item.type}'`,
				);
		}
		console.log(JSON.stringify([item, value]));
	}
	console.log(JSON.stringify(config));
	return config;
};

// called by js.rs
function $$invokeEvent(event, needsConfigReload) {
	if (needsConfigReload) {
		console.log("Reloading configuration file");
		reloadConfig();
	}

	for (let i = 0; i < $$IDS.length; i++) {
		const id = $$IDS[i];
		const callback = $$CALLBACKS[id];
		const config = $$CONFIG[id];

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

// called by js.rs
function $$getConfigSchema(event) {
	const result = [];

	for (let i = 0; i < $$IDS.length; i++) {
		const id = $$IDS[i];
		const name = $$NAMES[id];
		const description = $$DESCRIPTIONS[id];
		const config_schema = $$CONFIG_SCHEMAS[id];

		result.push({
			id: id,
			name: name,
			description: description,
			config: config_schema,
		});
	}
	const json = JSON.stringify({
		plugins: result,
	});
	console.log(`json=${json}`);
	return json;
}

// called by js.rs
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
