import { _, init, addMessages } from 'svelte-i18n';

import en from './dictionary/en.json';
import ja from './dictionary/ja.json';

function setupI18n(defaultLocale: string) {
	addMessages('en', en);
	addMessages('ja', ja);

	init({
		fallbackLocale: 'en',
		initialLocale: defaultLocale
	});
}

export { _, setupI18n };
