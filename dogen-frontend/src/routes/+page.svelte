<script lang="ts">
	import { generateMap } from '$lib/generator';
	import type { MapData } from '$lib/map';
	import { onMount } from 'svelte';

	let mapData: MapData | null = null;

	let cityName: string = '';

	onMount(reloadMap);

	async function reloadMap() {
		mapData = await generateMap();
		cityName = mapData.map.get_nameset().city_name().name();
	}
</script>

<div id="map" />

<div id="control">
	<div class="cityname">
		{cityName || ''}
		<span class="citypostfix">{cityName ? '市街' : ''}</span>
	</div>
	<button on:click={reloadMap}>Regenerate</button>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
	}

	#map {
		width: 50vw;
		height: 100vh;
		background-color: #f8f8f8;
	}

	#control {
		width: 50vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.cityname {
		font-size: 2rem;
	}

	.citypostfix {
		font-size: 1rem;
	}
</style>
