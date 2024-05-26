<script lang="ts">
	import Introduction from "../components/introduction.svelte";
	
    import { _ } from 'svelte-i18n';
	import init from "$lib/engine/hokkaido_generator";
	import { initialSeedStore, mapSetStore, placenameDatasetStore } from "../routes/store";
	import { createMap } from "$lib/map";
	import { onMount } from "svelte";
    
    let view3D = false;
    let nightMode = false;

	let placenameDataset = '';
	placenameDatasetStore.subscribe(value => {
		placenameDataset = value;
	});

	let isLoading = false;
	let mapSet = undefined;
	mapSetStore.subscribe(value => {
		mapSet = value;
		isLoading = false;
	});

    async function generateNew() {
		isLoading = true;
		await init();
		setTimeout(() => {
			mapSetStore.set(createMap(placenameDataset, view3D, nightMode, undefined));
		}, 0);
    }

	onMount(() => {
		// generate new map if initialSeed is set
		initialSeedStore.subscribe(value => {
			if (value !== undefined) {
				generateNew();
			}
		});
	});

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
		<input type="checkbox" id="presentation" bind:checked={nightMode} />
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