<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {onMount} from "svelte";

    export let pluginId

    let pluginConfig = {
        enabled: false,
        config: {},
    };
    let configSchema = {
    };

    onMount(async () => {
        console.log(pluginId);
        configSchema = await invoke("get_config_schema_for_plugin", {pluginId});
        pluginConfig = await invoke("load_config_for_plugin", {pluginId});
    });

    async function onChange() {
        await invoke("save_config_for_plugin", {
            pluginId,
            pluginConfig,
        })
    }
</script>

<div class="plugin-config">
    <h2>{configSchema.name}</h2>
    <div class="plugin-id">(<span class="id">{configSchema.id}</span>)</div>
    <div class="description">{configSchema.description}</div>
    <div class="enabled">
        <label>
            Enabled:
            <input type="checkbox" bind:checked={pluginConfig.enabled} on:change={onChange}>
        </label>
    </div>
    {#if pluginConfig.enabled}
        <table>
            {#each configSchema.config as schema_config}
                <tr class="config">
                    <th>{schema_config.name}<br>(<span class="type">{schema_config.type}</span>)</th>
                    <td>
                        <input type="text" bind:value={pluginConfig.config[schema_config.name]} on:change={onChange}>
                        <div class="description">{schema_config.description}</div>
                        <div class="default">Default: {schema_config.default}</div>
                    </td>
                </tr>
            {/each}
        </table>
    {/if}
</div>

<style>
    .plugin-config > .description {
        margin-bottom: 8px;
        padding: 9px;
        background-color: darkslategray;
    }

    th, td {
        border-top: 1px solid white;
        border-bottom: 1px solid white;
    }

    th {
        text-align: left;
    }

    .plugin-id {
        color: darkgrey;
    }
</style>