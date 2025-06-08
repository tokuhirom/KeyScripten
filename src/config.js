import "./styles.css";

import App from "./App.svelte";
import { mount } from "svelte";

const appElem = document.getElementById("app");
if (!appElem) {
	throw new Error("cannot detect #app element");
}
const app = mount(App, { target: appElem });

export default app;
