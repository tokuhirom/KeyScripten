<script>

    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/tauri";

    let event_log = [];

    async function updateEventLog() {
        const r = await invoke("get_event_log", {});
        r.reverse();
        event_log = r;
    }

    onMount(async () => {
        await updateEventLog();
        setInterval(updateEventLog, 1000);
    });
</script>
<div>
    <table>
        <tr><th>Event type</th><th>KeyCode</th><th>Flags</th></tr>
    {#each event_log as log}
        <tr><td>{log.event_type}</td><td>{log.keycode}</td><td>{log.flags}</td></tr>
    {/each}
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
