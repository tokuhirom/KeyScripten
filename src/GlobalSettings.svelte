<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {onMount} from "svelte";

    let config = {
        log_level: "info",
    };

    onMount(async () => {
        const c = await invoke("load_config");
        c.log_level ||= "info";
        config = c;
    });

    async function handleChangeLogLevel() {
        console.log(`You selected: ${config.log_level}`);
        await invoke("update_log_level", {
            logLevel: config.log_level,
        });
    }
</script>

<div>
    <h2>Global settings</h2>
    <table>
        <tr>
            <th>Log Level</th>
            <td>
                <div>
                    <select bind:value="{config.log_level}" on:change={handleChangeLogLevel}>
                        <option value="info">Info</option>
                        <option value="debug">Debug</option>
                    </select>
                </div>
                If you set the log level to debug or lower, the log file may contain your personal information
                and/or credential info. Take carefully.
            </td>
        </tr>
    </table>
</div>
