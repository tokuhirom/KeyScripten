<script>
import { invoke } from "@tauri-apps/api/tauri";
import { afterUpdate, onMount } from "svelte";
import { emit } from "@tauri-apps/api/event";
import UpdatedNotice from "./UpdatedNotice.svelte";

export let pluginId;
let prevPluginId;
let code;
let message;
let showMessage = false;

async function reload() {
	if (pluginId === prevPluginId && !!prevPluginId) {
		console.log(`No pluginId modification: ${pluginId}, ${prevPluginId}`);
		return;
	}

	prevPluginId = pluginId;
	code = await invoke("read_plugin_code", { pluginId });
	return false;
}

onMount(reload);
afterUpdate(reload);

async function submit() {
	await invoke("write_plugin_code", { pluginId, code });
	await emit("js-operation", {
		ReloadPlugins: null,
	});
	message = `saved ${pluginId} at ${getCurrentTimeInHHMMSS()}`;
	showMessage = true;
	return false;
}

function getCurrentTimeInHHMMSS() {
	const now = new Date(); // 現在の日時を取得

	const hours = String(now.getHours()).padStart(2, "0");
	const minutes = String(now.getMinutes()).padStart(2, "0");
	const seconds = String(now.getSeconds()).padStart(2, "0");

	return `${hours}:${minutes}:${seconds}`;
}
</script>

<form on:submit|preventDefault={submit}>
    <UpdatedNotice message={message} showMessage={showMessage} />
    <textarea bind:value={code}></textarea>
    <button type="submit">Save</button>
</form>

<style>
    form {
        display: flex;
        flex-direction: column;
        height: 70vh;
    }
    form textarea {
        flex-grow: 1;
    }
</style>