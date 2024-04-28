<script lang="ts">
	import { generateMap } from '$lib/generator';
	import { onMount } from 'svelte';

	onMount(async () => {
		await generateMap();
	});

	async function reloadMap() {
		const map = document.getElementById('map');
		if (map === null) {
			return;
		}
		map.innerHTML = '';
		await generateMap();
	}
</script>

<div id="map" />

<div>
	<button on:click={reloadMap}>Regenerate</button>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
	}

	#map {
		width: 80vmin;
		height: 80vh;
		background-color: #f8f8f8;
	}
</style>
