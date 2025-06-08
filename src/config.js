import "./styles.css";

import { mount } from "svelte";
import App from "./App.svelte";

const appElem = document.getElementById("app");
if (!appElem) {
	throw new Error("cannot detect #app element");
}
const app = mount(App, { target: appElem });

export default app;
