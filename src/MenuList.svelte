<script>
    import {afterUpdate} from "svelte";

    export let pane = "settings";
    export let plugins;
    export let onPaneChange;

    afterUpdate(() => {
        if (pane.startsWith("plugin:")) {
            let pluginId = pane.replace("plugin:", "");
            for (let plugin of plugins) {
                if (pluginId === plugin.id) {
                    return;
                }
            }

            console.log("the selected plugin is no longer available.");
            pane = "settings";
            onPaneChange(pane);
        }
    });

    function showKeyEvents() {
        pane = "keyEvents";
        onPaneChange(pane);
    }
    function showLogViewer() {
        pane = "logViewer";
        onPaneChange(pane);
    }
    function showSettings() {
        pane = "settings";
        onPaneChange(pane);
    }
    function showPluginPane(id) {
        pane = "plugin:" + id;
        onPaneChange(pane);
        return false;
    }

    function addPlugin() {
        pane = "addPlugin";
        onPaneChange(pane);
    }
</script>

<div class="sidebar">
    <menu>
        <li><button class:selected="{pane === 'keyEvents'}"
                    on:click={showKeyEvents}>Key Events</button></li>
        <li><button class:selected="{pane === 'logViewer'}"
                    on:click={showLogViewer}>Log Viewer</button></li>
        <li><button class:selected="{pane === 'settings'}"
                    on:click={showSettings}>Settings</button></li>
        {#each plugins as plugin}
            <li><button class:selected="{pane === 'plugin:' + plugin.id}" on:click={() => showPluginPane(plugin.id)}>{plugin.name}</button></li>
        {/each}
    </menu>
    <div class="add-plugin">
        <button class:selected="{pane === 'addPlugin'}" on:click={addPlugin}>Add plugin</button>
    </div>
</div>

<style>
    button {
        margin: 4px;
        width: 180px;
    }

    .sidebar {
        display: flex;
        flex-direction: column;
        height: 90vh;
    }

    menu {
        list-style-type: none;
        padding-inline-start: 0;
        flex-grow: 1;
        margin-block-start: 0;
        margin-block-end: 0;
    }
    menu button {
        margin-top: 4px;
    }
    menu .selected {
        background-color: #396cd8;
    }

    .add-plugin {
        margin-top: auto;
    }
</style>
