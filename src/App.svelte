<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import {onMount} from "svelte";

    let config = {};
    let config_schema = {
        plugins: []
    };

    onMount(async () => {
        config_schema = await invoke("get_config_schema");
        config = await invoke("load_config");
    });
</script>

<main class="container">
    <h1>Configuration for MaguroMate</h1>

    {#each config_schema.plugins as schema}
        {schema.id}
        <table>
            {#each schema.config as config}
                <tr>
                    <th>{config.name}({config.type})</th>
                    <td>Default: {config.default}</td>
                </tr>
            {/each}
        </table>
    {/each}

    <div style="background-color: bisque; color: black">
        <h2>Config schema</h2>
        {JSON.stringify(config_schema)}
        <h2>Config</h2>
        {JSON.stringify(config)}
    </div>
</main>

<style>
</style>