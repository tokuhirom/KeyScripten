<script>
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { onMount } from "svelte";
import AddPlugin from "./AddPlugin.svelte";
import ConsoleLog from "./ConsoleLog.svelte";
import ErrorScreen from "./ErrorScreen.svelte";
import EventLog from "./EventLog.svelte";
import Settings from "./GlobalSettings.svelte";
import LogViewer from "./LogViewer.svelte";
import MenuList from "./MenuList.svelte";
import PluginDetails from "./PluginDetails.svelte";

let config_schema = {
	plugins: [],
};
let pane = "settings";
let setupError = null;
let appReady = false;

onMount(async () => {
	// First check for error state
	const error = await invoke("get_setup_error");
	if (error) {
		setupError = error;
	}

	// If no error, proceed with normal initialization
	if (!setupError) {
		config_schema = await invoke("get_config_schema");
		await listen("config_schema-reload", async () => {
			config_schema = await invoke("get_config_schema");
		});
	}

	appReady = true;
});

/**
 * @param {string} pane_
 */
function onPaneChange(pane_) {
	pane = pane_;
}
</script>

<div>
    {#if appReady}
        {#if setupError}
            <ErrorScreen errorMessage={setupError} />
        {:else}
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
        {/if}
    {:else}
        <div class="loading">
            <p>Loading application...</p>
        </div>
    {/if}
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

    .loading {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
        font-size: 18px;
        color: #555;
    }
</style>
