<script lang="ts">
	import Introduction from '../components/introduction.svelte';

	import { _ } from 'svelte-i18n';
	import init from '$lib/engine/hokkaido_generator';
	import { initialSettingsStore, mapSetStore, placenameDatasetStore } from '../routes/store';
	import { MapSet, createMap } from '$lib/map';
	import { onMount } from 'svelte';
	import Cityinfo from './cityinfo.svelte';

	let seed: number;
	let view3D = false;
	let darkMode = false;
	let locale = '';

	let isLoading = false;
	let mapSet: MapSet | undefined = undefined;

	$: if (mapSet)
		history.replaceState(
			null,
			'',
			`/?seed=${seed}&view3D=${view3D}&darkMode=${darkMode}&locale=${locale}`
		);

	onMount(() => {
		initialSettingsStore.subscribe((initialSettings) => {
			if (initialSettings.view3D !== undefined) {
				view3D = initialSettings.view3D;
			}
			if (initialSettings.darkMode !== undefined) {
				darkMode = initialSettings.darkMode;
			} else {
				darkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
			}
			locale = initialSettings.locale;
			if (initialSettings.seed !== undefined) {
				generateNew(initialSettings.seed);
			} else {
				mapSetStore.set(undefined);
			}
		});
		mapSetStore.subscribe((value) => {
			mapSet = value;
			isLoading = false;
		});
	});

	let placenameDataset = '';
	placenameDatasetStore.subscribe((value) => {
		placenameDataset = value;
	});

	async function generateNew(presetSeed?: number) {
		if (presetSeed) {
			seed = presetSeed;
		} else {
			seed = Math.floor(Math.random() * 1000000) + 1;
		}
		isLoading = true;
		await init();
		setTimeout(() => {
			mapSetStore.set(createMap(placenameDataset, view3D, darkMode, seed));
		}, 0);
	}

	// update view when darkMode or view3D changes
	$: if (mapSet) {
		mapSet.updateView(view3D, darkMode);
		if (darkMode) {
			document.documentElement.classList.add('dark');
			document.documentElement.classList.remove('light');
			localStorage.setItem('nightMode', 'true');
		} else {
			document.documentElement.classList.remove('dark');
			document.documentElement.classList.add('light');
			localStorage.setItem('nightMode', 'false');
		}
	}
</script>

<div id="control">
	{#if mapSet}
		<Cityinfo {mapSet} {seed} {locale} />
	{:else}
		<Introduction />
	{/if}
	{#key isLoading}
		<button on:click={() => generateNew()} id="generateButton" disabled={isLoading}>
			{#if isLoading}
				{$_('loading')}
			{:else}
				{$_('generate-new')}
			{/if}
		</button>
	{/key}

	<div id="checkbox">
		<input type="checkbox" id="presentation" bind:checked={view3D} />
		{$_('3d-terrain')}
	</div>

	<div id="checkbox">
		<input type="checkbox" id="presentation" bind:checked={darkMode} />
		{$_('dark-mode')}
	</div>
</div>

<style>
	#control {
		flex: 1;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		text-align: center;
		gap: 0.5rem;
	}

	#generateButton {
		margin-top: 1rem;
		font-weight: bold;
	}

	#checkbox {
		color: var(--sub-text);
		font-size: 0.9rem;
	}

	button {
		border: none;
		background-color: var(--button-bg);
		color: var(--button-text);
		border-radius: 0.5rem;
		padding: 0.5rem 1rem;
	}

	button:hover {
		background-color: var(--button-bg-hover);
	}
</style>
