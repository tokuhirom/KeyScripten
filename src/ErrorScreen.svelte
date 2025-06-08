<script>
export let errorMessage;
</script>

<div class="error-container">
    <div class="error-box">
        <h2>Cannot Capture Keyboard Events</h2>

        <div class="error-message">
            <p>KeyScripten requires macOS "Accessibility" permission to run properly.</p>
            <p>Please follow these steps to configure it:</p>

            <ol>
                <li>Open "System Preferences"</li>
                <li>Click on "Security & Privacy"</li>
                <li>Select the "Privacy" tab</li>
                <li>Select "Accessibility" from the list on the left</li>
                <li>Check the box next to "KeyScripten" in the list on the right</li>
                <li>Restart the application</li>
            </ol>

            <div class="technical-details">
                <h3>Technical Details:</h3>
                <pre>{errorMessage}</pre>
            </div>
        </div>

        <div class="instructions">
            <img src="../img_3.png" alt="Accessibility Settings Screen" class="screenshot" />
            <img src="../img_4.png" alt="Accessibility Permission Settings" class="screenshot" />
        </div>
    </div>
</div>

<style>
    .error-container {
        display: flex;
        justify-content: center;
        align-items: flex-start;
        height: 100%;
        padding: 20px;
    }

    .error-box {
        background-color: #fff;
        border: 1px solid #e74c3c;
        border-radius: 8px;
        padding: 20px;
        max-width: 800px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    }

    h2 {
        color: #e74c3c;
        margin-top: 0;
        text-align: center;
        border-bottom: 1px solid #eee;
        padding-bottom: 10px;
    }

    .error-message {
        margin-bottom: 20px;
        color: #0f0f2f;
    }

    ol {
        margin-left: 20px;
        padding-left: 0;
    }

    li {
        margin-bottom: 8px;
    }

    .technical-details {
        background-color: #f8f9fa;
        border: 1px solid #ddd;
        border-radius: 4px;
        padding: 10px;
        margin-top: 20px;
    }

    .technical-details h3 {
        margin-top: 0;
        font-size: 14px;
        color: #555;
    }

    pre {
        white-space: pre-wrap;
        word-break: break-all;
        margin: 0;
        font-size: 12px;
        color: #666;
    }

    .instructions {
        margin-top: 20px;
    }

    .screenshot {
        width: 100%;
        border: 1px solid #ddd;
        border-radius: 4px;
        margin-top: 10px;
    }
</style>
