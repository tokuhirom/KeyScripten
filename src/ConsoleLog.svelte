<script>

    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";

    let console_log = [];

    async function updateConsoleLog() {
        const r = await invoke("read_console_logs", {});
        r.reverse();
        console_log = r;
    }

    let intervalId;

    onMount(async () => {
        await updateConsoleLog();

        // TODO fetching events by setInterval is not so good.
        // i want to push events from rust side, but it's bit hard.
        intervalId = setInterval(updateConsoleLog, 1000);
    });
    onDestroy(() => {
        clearInterval(intervalId);
    });

    function formatEpochToHHMMSS(epochSeconds) {
        let date = new Date(epochSeconds * 1000);
        let hours = date.getHours().toString().padStart(2, '0');
        let minutes = date.getMinutes().toString().padStart(2, '0');
        let seconds = date.getSeconds().toString().padStart(2, '0');

        return `${hours}:${minutes}:${seconds}`;
    }
</script>
<div>
    <p>Javascript's console log(periodically fetched from application core)</p>
    <table>
        <thead>
            <tr><th>Time</th><th>Log level</th><th>message</th></tr>
        </thead>
        <tbody>
    {#each console_log as log}
        <tr>
            <td>{formatEpochToHHMMSS(log.time_seconds)}</td>
            <td>{log.level}</td>
            <td>{log.message}</td>
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
</style>
