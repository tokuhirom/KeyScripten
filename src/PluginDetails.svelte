<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {afterUpdate, onMount} from "svelte";
    import {emit} from "@tauri-apps/api/event";
    import PluginSettings from "./PluginSettings.svelte";
    import EditPluginCode from "./EditPluginCode.svelte";
    import PluginOther from "./PluginOther.svelte";

    export let pluginId
    let prevPluginId;
    let filename = undefined;

    let tab = "settings";

    let pluginConfig = {
        enabled: false,
        config: {},
    };
    let configSchema = {
        id: undefined,
        config: [],
    };

    async function reload() {
        if (pluginId === prevPluginId && !!prevPluginId) {
            console.log(`No pluginId modification: ${pluginId}, ${prevPluginId}`)
            return;
        }

        prevPluginId = pluginId;

        configSchema = await invoke("get_config_schema_for_plugin", {pluginId});
        pluginConfig = await invoke("load_config_for_plugin", {pluginId});
        if (!pluginId.startsWith("bundled.")) {
            filename = await invoke("get_plugin_filename", {pluginId});
        }
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

    function showEdit() {
        tab = "edit";
    }
    function showSettings() {
        tab = "settings";
    }
    function showOther() {
        tab = "other";
    }
</script>

<div class="plugin-config">
    <h2>{configSchema.name}</h2>
    <div class="plugin-id">(<span class="id">{configSchema.id}</span>)</div>
    {#if filename}
        <div class="plugin-filename">{filename}</div>
    {/if}
    <div class="description">{configSchema.description}</div>
    {#if configSchema.id && !configSchema.id.startsWith("builtin.")}
    <menu>
        <li><button class:selected="{tab === 'settings'}"
                    on:click={showSettings}>Settings</button></li>
        <li><button class:selected="{tab === 'edit'}"
                    on:click={showEdit}>Edit</button></li>
        <li><button class:selected="{tab === 'other'}"
                    on:click={showOther}>Other</button></li>
    </menu>
    {/if}
    <div class="tab-content">
    {#if tab === "settings"}
        <PluginSettings configSchema={configSchema} pluginId={pluginId} />
    {:else if tab === "edit"}
        <EditPluginCode pluginId={pluginId} />
    {:else}
        <PluginOther pluginId={pluginId} />
    {/if}
    </div>
</div>

<style>
    .plugin-config > .description {
        margin-bottom: 8px;
        padding: 9px;
        background-color: darkslategray;
    }
    menu {
        display: flex;
        flex-direction: row;
        padding: 0;
        margin: 0;
    }
    menu li {
        list-style: none;
        margin-right: 2px;
        background-color: #a9a9a9;
    }
    menu button {
        border-radius: 2px 2px 0 0;
    }
    menu button.selected {
        background-color: #0f0f2f;
    }
    .tab-content {
        background-color: #0f0f2f;
        padding: 8px;
        border-bottom-left-radius: 2px;
        border-bottom-right-radius: 2px;
    }
</style>
