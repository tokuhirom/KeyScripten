<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import {onMount} from "svelte";
    import Settings from "./GlobalSettings.svelte";
    import MenuList from "./MenuList.svelte";
    import EventLog from "./EventLog.svelte";
    import AddPlugin from "./AddPlugin.svelte";
    import PluginDetails from "./PluginDetails.svelte";
    import {listen} from "@tauri-apps/api/event";
    import LogViewer from "./LogViewer.svelte";
    import ConsoleLog from "./ConsoleLog.svelte";

    let config_schema = {
        plugins: []
    };
    let pane = "settings";

    onMount(async () => {
        config_schema = await invoke("get_config_schema");
        await listen('config_schema-reload', async () => {
            config_schema = await invoke("get_config_schema");
        })
    });

    /**
     * @param {string} pane_
     */
    function onPaneChange(pane_) {
        pane = pane_;
    }
</script>

<div>
    <div class="container">
        <div class="menu">
            <MenuList pane={pane} plugins={config_schema.plugins} onPaneChange={onPaneChange} />
        </div>

        <div class="content">
            {#if pane==="settings"}
                <Settings />
            {:else if pane.startsWith("plugin:")}
                <PluginDetails pluginId={pane.replace("plugin:", "")} />
            {:else if pane === "logViewer"}
                <LogViewer />
            {:else if pane === "console"}
                <ConsoleLog />
            {:else if pane === "keyEvents"}
                <EventLog />
            {:else if pane === "addPlugin"}
                <AddPlugin />
            {:else}
                Unknown pane: {pane}
            {/if}
        </div>
    </div>
</div>

<style>
    .container {
        display: flex;
    }

    .menu {
        width: 190px;
    }

    .content {
        margin-left: 10px;
        flex-grow: 1;
    }
</style>
