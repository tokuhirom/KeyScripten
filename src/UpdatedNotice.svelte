<script>
export let message = "";

// メッセージの表示状態を管理するストア
export let showMessage;

// メッセージが更新されたときにトリガーされる
$: if (message) {
	showMessage = true;
	setTimeout(() => {
		console.log("hide message");
		showMessage = false;
		message = "";
	}, 2000);
}
</script>

<div class:visible={showMessage} class:hidden={!showMessage}>
    {message}
</div>

<style>
    .visible {
        display: block;
        background-color: yellowgreen;
        padding: 8px;
    }

    .hidden {
        display: none;
    }
</style>
