<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import {onMount} from "svelte";
    import Settings from "./GlobalSettings.svelte";
    import PluginSettings from "./PluginSettings.svelte";
    import MenuList from "./MenuList.svelte";

    let config_schema = {
        plugins: []
    };
    let pane = "settings";

    onMount(async () => {
        config_schema = await invoke("get_config_schema");
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
                <PluginSettings pluginId={pane.replace("plugin:", "")} />
            {/if}
        </div>
    </div>
</div>

<style>
    h2, h3 {
        text-align: left;
    }

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
