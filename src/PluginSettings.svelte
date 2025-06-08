<script>
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";
import { afterUpdate, onMount } from "svelte";

export let pluginId;
export const configSchema = {
	config: [],
};
let prevPluginId;
let prevConfigSchema;

let pluginConfig = {
	enabled: false,
	config: {},
};

async function reload() {
	if (
		pluginId === prevPluginId &&
		configSchema === prevConfigSchema &&
		!!prevPluginId
	) {
		console.log(`No pluginId modification: ${pluginId}, ${prevPluginId}`);
		return;
	}
	console.log(`Loading plugin configuration: ${pluginId}`);

	prevPluginId = pluginId;
	prevConfigSchema = configSchema;

	pluginConfig = await invoke("load_config_for_plugin", { pluginId });
	for (const option of configSchema.config) {
		if (!(option.name in pluginConfig.config)) {
			pluginConfig.config[option.name] = option.default;
		}
		console.log(pluginConfig);
	}
	console.log(
		`Loaded plugin configuration: pluginId=${pluginId}, pluginConfig=${JSON.stringify(pluginConfig)}, configSchema=${JSON.stringify(configSchema)}`,
	);
}

onMount(async () => {
	console.log(pluginId);
	await reload();
});

afterUpdate(async () => {
	await reload();
});

async function onChange() {
	await invoke("save_config_for_plugin", {
		pluginId,
		pluginConfig,
	});
	await emit("js-operation", {
		ReloadConfig: null,
	});
}
</script>

<div class="plugin-config">
    <div class="enabled">
        <label>
            Enabled:
            <input type="checkbox" bind:checked={pluginConfig.enabled} on:change={onChange}>
        </label>
    </div>
    {#if pluginConfig.enabled}
        <table class="plugin-config-detail">
            <tbody>
            {#each configSchema.config as schema_config}
                <tr class="config">
                    <th>{schema_config.name}<br>(<span class="type">{schema_config.type}</span>)</th>
                    <td>
                        <input type="text" bind:value={pluginConfig.config[schema_config.name]} on:change={onChange}>
                        <div class="description">{schema_config.description}</div>
                        {#if schema_config.type === "hotkey"}
                            <div class="hotkey-note">
                                <table>
                                    <tbody>
                                    <tr>
                                        <th>C-</th>
                                        <td>means (press and hold) the Control key</td>
                                    </tr>
                                    <tr>
                                        <th>M-</th>
                                        <td>Cmd key(M-, means cmd-comma)</td>
                                    </tr>
                                    <tr>
                                        <th>S-</th>
                                        <td>Shift key(S-tab means shift-tab)</td>
                                    </tr>
                                    </tbody>
                                </table>
                            </div>
                        {/if}
                        <div class="default">Default: {schema_config.default}</div>
                    </td>
                </tr>
            {/each}
            </tbody>
        </table>
    {/if}
</div>

<style>
    table.plugin-config-detail {
        border-collapse: collapse;
        border-radius: 8px;
    }
    table.plugin-config-detail, table.plugin-config-detail > tbody > tr > th, table.plugin-config-detail > tbody > tr > td {
        border: 1px solid white;
        padding: 4px;
    }

    th {
        text-align: left;
    }

    .hotkey-note {
        background-color: dimgray;
        margin-left: 8px;
        border-radius: 3px;
        padding-left: 4px;
    }
</style>
