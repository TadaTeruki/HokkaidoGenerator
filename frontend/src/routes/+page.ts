import { setupI18n } from "$lib/translation";
import { initialSeedStore, placenameDatasetStore } from "./store";

/** @type {import('./$types').PageLoad} */
export async function load(event: any) {
    const params = new URLSearchParams(event.url.search);
    const localeSrc = params.get('locale');
    const locale = localeSrc ? localeSrc : 'ja';
    setupI18n(locale);

    const initialSeed = params.get('seed');
    if (initialSeed) {
        initialSeedStore.set(parseInt(initialSeed));
    }

    await event.fetch('/dataset/placenames.csv').then((responses: any) => {
        return responses.text();
    }).then((dataset: string) => {
        placenameDatasetStore.set(dataset);
    });
}
