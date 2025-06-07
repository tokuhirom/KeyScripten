import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

export default {
    // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
    // for more information about preprocessors
    preprocess: vitePreprocess(),

    // Enable Svelte 5's backward compatibility mode
    // https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes
    compilerOptions: {
        compatibility: {
            componentApi: 4
        }
    }
};