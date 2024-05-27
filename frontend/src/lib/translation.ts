import { _, init, addMessages, getLocaleFromNavigator } from 'svelte-i18n';

import en from './dictionary/en.json';
import ja from './dictionary/ja.json';

function setupI18n(defaultLocale: string | undefined) {
	addMessages('en', en);
	addMessages('ja', ja);

	const initialLocale = defaultLocale || getLocaleFromNavigator();

	init({
		fallbackLocale: 'en',
		initialLocale: initialLocale
	});
}

export { _, setupI18n };
