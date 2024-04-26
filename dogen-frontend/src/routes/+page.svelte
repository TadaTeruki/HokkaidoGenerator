<script lang="ts">
	//import { MapLibre } from 'svelte-maplibre';
	import init from '$lib/engine/dogen_generator';
	import { MapData } from '$lib/map';

	async function onload() {
		console.log('onload');

		await init();

		const width = 1200;
		const height = 600;
		const seed = Math.floor(Math.random() * 1000);

		const mapData = new MapData(seed, width / height);

		const canvas = document.getElementById('mapcanvas') as HTMLCanvasElement;
		canvas.width = width;
		canvas.height = height;

		console.log('done');

		mapData.drawTerrain(canvas);
	}
</script>

<svelte:window on:click={onload} />

<canvas id="mapcanvas"></canvas>

<style>
</style>
