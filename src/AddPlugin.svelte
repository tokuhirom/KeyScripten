<script>
import { emit } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

let name;
let pluginId;
let description;

async function addPlugin() {
	await invoke("add_plugin", {
		pluginId,
		name,
		description,
	});
	await emit("js-operation", {
		ReloadPlugins: null,
	});
	return false;
}
</script>

<div>
    <h2>Add Plugin</h2>
    <form on:submit={addPlugin}>
        <label for="pluginId">PluginId</label>
        <input type="text" id="pluginId" bind:value={pluginId} required />

        <label for="name">Name</label>
        <input type="text" id="name" bind:value={name} required />

        <label for="description">Description</label>
        <textarea id="description" bind:value={description} required></textarea>

        <button type="submit">Add new plugin</button>
    </form>
</div>

<style>
    form {
        display: grid;
        grid-template-columns: auto 1fr;
        gap: 10px;
        align-items: center;
        width: 100%;
    }

    label {
        text-align: right;
    }

    textarea {
        grid-column: 1 / span 2; /* Make textarea span across two columns */
    }

    button {
        grid-column: 1 / span 2; /* Make button span across two columns */
        margin-top: 10px;
    }
</style>
