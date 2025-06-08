<script>
import { invoke } from "@tauri-apps/api/core";
import { onDestroy, onMount } from "svelte";
import { flagsToString } from "./flags.js";
import { getKeyName } from "./keycode.js";

let logs = [];

async function updateEventLog() {
	const r = await invoke("read_logs", {});
	r.reverse();
	logs = r;
}

let intervalId;

onMount(async () => {
	await updateEventLog();
	intervalId = setInterval(updateEventLog, 1000);
});
onDestroy(() => {
	clearInterval(intervalId);
});

function formatEpochToHHMMSS(epochSeconds) {
	const date = new Date(epochSeconds * 1000);
	const hours = date.getHours().toString().padStart(2, "0");
	const minutes = date.getMinutes().toString().padStart(2, "0");
	const seconds = date.getSeconds().toString().padStart(2, "0");

	return `${hours}:${minutes}:${seconds}`;
}

function getLogLevel(logMessage) {
	const levelMatch = logMessage.match(/ ([A-Z]+) \S+]/);
	return levelMatch ? levelMatch[1].toLowerCase() : "unknown";
}
</script>
<div>
    <table>
        <thead>
            <tr><th>Message</th></tr>
        </thead>
        <tbody>
    {#each logs as log}
        <tr class={getLogLevel(log)}>
            <td>{log}</td>
        </tr>
    {/each}
        </tbody>
    </table>
</div>

<style>
    table, th, td {
        border: 1px solid cadetblue;
        padding: 4px;
    }

    table {
        border-collapse: collapse;
    }

    .info {
        color: green;
    }

    .warn {
        color: orange;
    }

    .error {
        color: red;
    }

    .debug {
        color: blue;
    }

    .unknown {
        color: grey;
    }
</style>
