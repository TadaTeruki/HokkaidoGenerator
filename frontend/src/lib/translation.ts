import { _, init, getLocaleFromNavigator, addMessages } from 'svelte-i18n';

import en from './dictionary/dictionary-en.json';
import ja from './dictionary/dictionary-ja.json';

function setupI18n(defaultLocale: string) {
	addMessages('en', en);
	addMessages('ja', ja);

	init({
		fallbackLocale: 'en',
		initialLocale: defaultLocale
	});
}

export { _, setupI18n };
