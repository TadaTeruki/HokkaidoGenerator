import { setupI18n } from "$lib/translation";
import { initialSettingsStore, placenameDatasetStore } from "./store";

/** @type {import('./$types').PageLoad} */
export async function load(event: any) {
    const params = new URLSearchParams(event.url.search);
    const localeSrc = params.get('locale');
    const locale = localeSrc ? localeSrc : 'ja';
    setupI18n(locale);

    const initialSeedSrc = params.get('seed');
    const initialSeed = function() {
        if (initialSeedSrc) {
            return parseInt(initialSeedSrc);
        } else {
            return undefined;
        }
    }

    initialSettingsStore.set({
        seed: initialSeed(),
        view3D: params.get('view3D') === 'true',
        darkMode: params.get('darkMode') === 'true'
    });

    await event.fetch('/dataset/placenames.csv').then((responses: any) => {
        return responses.text();
    }).then((dataset: string) => {
        placenameDatasetStore.set(dataset);
    });
}
