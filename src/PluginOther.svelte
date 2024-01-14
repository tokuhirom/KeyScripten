<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {afterUpdate, onMount} from "svelte";
    import {emit} from "@tauri-apps/api/event";
    import UpdatedNotice from "./UpdatedNotice.svelte";

    export let pluginId;

    async function deletePlugin() {
        await invoke("delete_plugin", {pluginId});
        await emit('js-operation', {
            "UnloadPlugin": {
                plugin_id: pluginId
            }
        });
        await emit("config_schema-reload");
        return false;
    }
</script>

<form on:submit|preventDefault={deletePlugin}>
    <button type="submit">Delete plugin</button>
</form>

<style>
</style>
