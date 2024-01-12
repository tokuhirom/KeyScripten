<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {afterUpdate, onMount} from "svelte";
    import {emit} from "@tauri-apps/api/event";

    export let pluginId
    export let configSchema;
    let prevPluginId;

    let pluginConfig = {
        enabled: false,
        config: {},
    };

    async function reload() {
        if (pluginId === prevPluginId && !!prevPluginId) {
            console.log(`No pluginId modification: ${pluginId}, ${prevPluginId}`)
            return;
        }

        prevPluginId = pluginId;

        pluginConfig = await invoke("load_config_for_plugin", {pluginId});
        for (const option of configSchema.config) {
            if (!(option.name in pluginConfig.config)) {
                pluginConfig.config[option.name] = option.default;
            }
            console.log(pluginConfig)
        }
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
        })
        await emit('update-config', "hello from front");
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
            {#each configSchema.config as schema_config}
                <tr class="config">
                    <th>{schema_config.name}<br>(<span class="type">{schema_config.type}</span>)</th>
                    <td>
                        <input type="text" bind:value={pluginConfig.config[schema_config.name]} on:change={onChange}>
                        <div class="description">{schema_config.description}</div>
                        {#if schema_config.type === "hotkey"}
                            <div class="hotkey-note">
                                <table>
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
                                </table>
                            </div>
                        {/if}
                        <div class="default">Default: {schema_config.default}</div>
                    </td>
                </tr>
            {/each}
        </table>
    {/if}
</div>

<style>
    table.plugin-config-detail {
        border-collapse: collapse;
        border-radius: 8px;
    }
    table.plugin-config-detail, table.plugin-config-detail > tr > th, table.plugin-config-detail > tr > td {
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