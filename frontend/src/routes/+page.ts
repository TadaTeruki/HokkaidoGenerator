import { setupI18n } from '$lib/translation';
import { getLocaleFromNavigator } from 'svelte-i18n';
import { initialSettingsStore, placenameDatasetStore } from './store';

/** @type {import('./$types').PageLoad} */
export async function load(event: any) {
	const params = new URLSearchParams(event.url.search);
	const localeSrc = params.get('locale');
	let locale = localeSrc ? localeSrc : getLocaleFromNavigator() || 'en';
	locale = locale.split('-')[0];
	setupI18n(locale);

	const initialSeedSrc = params.get('seed');
	const initialSeed = (function () {
		if (initialSeedSrc) {
			return parseInt(initialSeedSrc);
		} else {
			return undefined;
		}
	})();
	const view3DSrc = params.get('view3D');
	const view3D = (function () {
		if (view3DSrc) {
			return view3DSrc === 'true';
		} else {
			return undefined;
		}
	})();
	const darkModeSrc = params.get('darkMode');
	const darkMode = (function () {
		if (darkModeSrc) {
			return darkModeSrc === 'true';
		} else {
			return undefined;
		}
	})();

	initialSettingsStore.set({
		seed: initialSeed,
		view3D: view3D,
		darkMode: darkMode,
		locale: locale
	});

	await event
		.fetch('/dataset/placenames.csv')
		.then((responses: any) => {
			return responses.text();
		})
		.then((dataset: string) => {
			placenameDatasetStore.set(dataset);
		});
}
