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

	function shareText(cityNameRome: string, population: string) {
		const title = locale === 'ja' ? '北海道ジェネレータ' : 'Hokkaido Generator';
		const version = $_('version');
		const shareText = `${cityNameRome} (${population}) - ${title} ${version}`;
		return shareText;
	}

	function shareURL() {
		return `${window.location.origin}/?seed=${seed} #HokkaidoGenerator`;
	}

	function shareTextWithUrl(placeName: PlaceName) {
		const text =
			locale === 'ja'
				? shareText(placeName.cityNameKanji, placeName.populationJa)
				: shareText(placeName.cityNameRome, placeName.populationEn);
		return `${text} ${shareURL()}`;
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
