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
            showPane("settings");
        }
    });

    function showPane(p) {
        pane = p;
        onPaneChange(pane);
    }

    function addPlugin() {
        pane = "addPlugin";
        onPaneChange(pane);
    }
</script>

<div class="sidebar">
    <menu>
        <li><button class:selected="{pane === 'keyEvents'}"
                    on:click={() => showPane("keyEvents")}>Key Events</button></li>
        <li><button class:selected="{pane === 'logViewer'}"
                    on:click={() => showPane("logViewer")}>System Log Viewer</button></li>
        <li><button class:selected="{pane === 'console'}"
                    on:click={() => showPane("console")}>Console</button></li>
        <li class="general-menu"><button class:selected="{pane === 'settings'}"
                    on:click={() => showPane("settings")}>Settings</button></li>
        {#each plugins as plugin}
            <li class="plugin-menu"><button class:selected="{pane === 'plugin:' + plugin.id}"
                        on:click={() => showPane("plugin:" + plugin.id)}>{plugin.name}</button></li>
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

    menu .general-menu {
        margin-bottom: 18px;
    }

    .add-plugin {
        margin-top: auto;
    }
</style>
