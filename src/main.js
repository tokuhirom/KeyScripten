const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let greetMsg2El;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = JSON.stringify(await invoke("get_config_schema"));
  greetMsg2El.textContent = JSON.stringify(await invoke("load_config"));
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  greetMsg2El = document.querySelector("#greet-msg2");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
