<script>
    import {invoke} from "@tauri-apps/api/tauri";

    let config = {
        logLevel: "info",
    };

    async function handleChangeLogLevel() {
        console.log(`You selected: ${config.logLevel}`);
        await invoke("update_log_level", {
            log_level: config.logLevel,
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
                    <select bind:value="{config.logLevel}" on:change={handleChangeLogLevel}>
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
