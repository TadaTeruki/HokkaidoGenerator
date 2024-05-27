<script lang="ts">
	import type { MapSet, PlaceName } from '$lib/map';
	import CitynameEn from './citynameEn.svelte';
	import { _ } from 'svelte-i18n';
	import CitynameJa from './citynameJa.svelte';

	export let mapSet: MapSet | undefined = undefined;
	export let seed = 0;
	export let locale = '';

	let copiedSeed = -1;

	// image data URL
	let screenshot: string | undefined = undefined;

	function shareTextJa(cityNameKanji: string, population: string) {
		const shareText = `${cityNameKanji}市街 (${population}) - 北海道ジェネレータ`;
		return shareText;
	}

	function shareTextEn(cityNameRome: string, population: string) {
		const shareText = `${cityNameRome} (${population}) - Hokkaido Generator`;
		return shareText;
	}

	function shareURL() {
		return `${window.location.origin}/?seed=${seed} #HokkaidoGenerator`;
	}

	function shareText(placeName: PlaceName) {
		return locale === 'ja'
			? shareTextJa(placeName.cityNameKanji, placeName.populationJa)
			: shareTextEn(placeName.cityNameRome, placeName.populationEn);
	}

	function shareTextWithUrl(placeName: PlaceName) {
		return `${shareText(placeName)} ${shareURL()}`;
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		copiedSeed = seed;
	}
</script>

{#if locale === 'ja'}
	<CitynameJa {mapSet} />
{:else}
	<CitynameEn {mapSet} />
{/if}

<details class="share">
	<summary>{$_('share')}</summary>
	<p class="shareBox">
		{#if mapSet}
			{shareTextWithUrl(mapSet?.placeName)}<br />
			<button on:click={() => copyToClipboard(shareTextWithUrl(mapSet?.placeName))}
				>{$_('copy-to-clipboard')}</button
			>
			{copiedSeed === seed ? $_('copied') : ''}<br />
			<br />
			{#if screenshot}
				<img src={screenshot} alt={$_('screenshot-of-map')} />
			{:else}
				({$_('not-taken')})
			{/if}
			<br />
			<button on:click={() => (screenshot = mapSet.screenShot())}>{$_('take-screenshot')}</button
			><br />
		{/if}
	</p>
</details>

<style>
	.share {
		font-size: 0.8rem;
		color: var(--sub-text);
	}

	summary {
		text-decoration: underline;
	}

	.shareBox {
		background-color: var(--sub-bg);
		padding: 1rem;
		border-radius: 0.5rem;
		transition: background-color 0.5s;
	}

	.shareBox img {
		width: 20rem;
		height: 20rem;
		object-fit: contain;
		color: #888;
		margin-top: 0.5rem;
	}

	button {
		border: none;
		background-color: transparent;
		color: #888;
		border: 0.5px solid #888;
		border-radius: 0.3rem;
		margin-top: 0.5rem;
		padding: 0.1rem 0.5rem;
	}

	button:hover {
		background-color: #00000010;
	}
</style>
