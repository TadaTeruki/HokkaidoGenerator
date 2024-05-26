<script>
	import { _ } from 'svelte-i18n';
</script>

<div class="title">{$_('title')}</div>
<div class="subtitle">{$_('subtitle')}</div>
<div class="description">{$_('description')}</div>

<div class="credit">
	Copyright © 2024 Teruki TADA<br />

	Powered by <a href="https://www.svelte.dev">Svelte</a> and
	<a href="https://maplibre.org">Maplibre GL JS</a>
</div>

<div class="description">{$_('explanation-0')}</div>
<div class="description">{$_('explanation-1')}</div>

<style>
	.title {
		font-size: 3rem;
		color: var(--main-text);
	}

	.subtitle {
		font-size: 1rem;
		color: var(--sub-text);
	}

	.description {
		font-size: 1rem;
		color: var(--sub-text);
	}

	.credit {
		margin-top: 1rem;
		font-size: 1rem;
		color: var(--sub-text);
		line-height: 1.5;
	}

	a {
		text-decoration: none;
		color: inherit;
		border-bottom: 0.5px solid;
	}
</style>
