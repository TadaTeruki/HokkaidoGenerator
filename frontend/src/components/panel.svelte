<script lang="ts">
	import Introduction from "../components/introduction.svelte";
	
    import { _ } from 'svelte-i18n';
	import init from "$lib/engine/hokkaido_generator";
	import { initialSettingsStore, mapSetStore, placenameDatasetStore } from "../routes/store";
	import { MapSet, createMap } from "$lib/map";
	import { onMount } from "svelte";
    
	let seed: number | undefined = undefined;
    let view3D = false;
    let darkMode = false;
	onMount(() => {
		initialSettingsStore.subscribe(initialSettings => {
			if (initialSettings.view3D !== undefined) {
				view3D = initialSettings.view3D;
			}
			if (initialSettings.darkMode !== undefined) {
				darkMode = initialSettings.darkMode;
			}
			if (initialSettings.seed !== undefined) {
				seed = initialSettings.seed;
				generateNew();
			}
		});
	});

	let placenameDataset = '';
	placenameDatasetStore.subscribe(value => {
		placenameDataset = value;
	});

	let isLoading = false;
	let mapSet: MapSet | undefined = undefined;
	mapSetStore.subscribe(value => {
		mapSet = value;
		isLoading = false;
		seed = undefined;
	});

    async function generateNew() {
		isLoading = true;
		await init();
		setTimeout(() => {
			mapSetStore.set(createMap(placenameDataset, view3D, darkMode, seed));
		}, 0);
    }

	// update view when darkMode or view3D changes
	$: if (mapSet) {
		mapSet.updateView(view3D, darkMode);
	}

</script>

<div id="control">
	<Introduction />
	{#key isLoading}
	<button on:click={generateNew} id="generateButton" disabled={isLoading}>
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