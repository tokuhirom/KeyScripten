<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import {onMount} from "svelte";

    let config = {
        log_level: "info",
        plugins: []
    };
    let config_schema = {
        plugins: []
    };

    onMount(async () => {
        const cs = await invoke("get_config_schema");
        const c = await invoke("load_config");
        c.plugins ||= [];

        cs.plugins.forEach(plugin => {
            c.plugins[plugin.id] ||= {};
            c.plugins[plugin.id].config ||= {};
            if (!("enabled" in c.plugins[plugin.id])) {
                c.plugins[plugin.id].enabled = true;
            }
            c.plugins[plugin.id].config ||= {};
            for (let config_schema of plugin.config) {
                if (!(config_schema.name in c.plugins[plugin.id].config)) {
                    c.plugins[plugin.id].config[config_schema.name] = config_schema.default;
                }
            }

            console.log(plugin);
        })
        console.log(c);

        config = c;
        config_schema = cs;
    });

    function handleChangeLogLevel() {
        console.log(`You selected: ${config.log_level}`);
    }

    function handleSubmit() {
        alert("TODO: SAVED");
    }
</script>

<main class="container">
    <h1>Configuration for MaguroMate</h1>

    <form on:submit={handleSubmit}>
        <h2>Global configuration</h2>
        <table>
            <tr>
                <th>Log Level</th>
                <td>
                    <div>
                        <select bind:value="{config.log_level}" on:change={handleChangeLogLevel}>
                            <option value="info">Info</option>
                            <option value="debug">Debug</option>
                        </select>
                    </div>
                    If you set the log level to debug or lower, the log file may contain your personal information
                    and/or credential info. Take carefully.
                </td>
            </tr>
        </table>

        <h2>Plugin specific configuration</h2>
        {#each config_schema.plugins as schema}
            <div class="plugin-config">
                <h3>{schema.name}(<span class="id">{schema.id}</span>)</h3>
                <div class="description">{schema.description}</div>
                <div>
                    <label>
                        Enabled:
                        <input type="checkbox" bind:checked={config.plugins[schema.id].enabled}>
                    </label>
                </div>
                {config.plugins[schema.id].enabled}
                {#if config.plugins[schema.id].enabled}
                    <table>
                        {#each schema.config as schema_config}
                            <tr class="config">
                                <th>{schema_config.name}<br>(<span class="type">{schema_config.type}</span>)</th>
                                <td>
                                <input type="text" bind:value={config.plugins[schema.id].config[schema_config.name]}>
                                    <div class="description">{schema_config.description}</div>
                                    <div class="default">Default: {schema_config.default}</div>
                                </td>
                            </tr>
                        {/each}
                    </table>
                {/if}
            </div>
        {/each}

        <button type="submit">Save configuration</button>
    </form>

    <div style="background-color: bisque; color: black">
        <h2>Config schema</h2>
        <pre>{JSON.stringify(config_schema, null, 4)}</pre>
        <h2>Config</h2>
        <pre>{JSON.stringify(config, null, 4)}</pre>
    </div>
</main>

<style>
    h2, h3 {
        text-align: left;
    }

    .plugin-config {
        margin-left: 17px;
    }
    .plugin-config .id {
        color: cadetblue;
    }
    .plugin-config .description {
        text-align: left;
        color: darkgray;
        font-size: 80%;
    }
    .plugin-config table {
        margin-left: 8px;
    }
    .plugin-config tr .type {
        color: #24c8db;
    }
    .plugin-config tr .default {
        color: #24c8db;
    }
</style>