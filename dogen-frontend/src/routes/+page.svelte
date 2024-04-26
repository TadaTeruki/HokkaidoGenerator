<script lang="ts">
	import init from '$lib/engine/dogen_generator';
	import { MapData } from '$lib/map';
	import maplibre from 'maplibre-gl';
	import { onMount } from 'svelte';

	onMount(async () => {

		await init();
		
		const width = 500;
		const height = 500;
		const seed = Math.floor(Math.random() * 1000);

		const mapData = new MapData(seed, width / height);

		const canvas = document.createElement('canvas');
		canvas.width = width;
		canvas.height = height;

		mapData.drawTerrain(canvas);

		const url = canvas.toDataURL("image/jpeg");

		let map = new maplibre.Map({
			container: 'map',
			zoom: 5,
			minZoom: 4,
			center: [95.899147, 18.088694],
			style:
				'https://api.maptiler.com/maps/streets/style.json?key=get_your_own_OpIi9ZULNHzrESv6T2vL',
		});
		map.on('load', () => {
			map.addSource('canvas-source', {
				type: 'image',
				url,
				coordinates: [
					[91.4461, 21.5006],
					[100.3541, 21.5006],
					[100.3541, 13.9706],
					[91.4461, 13.9706]
				],
			});

			map.addLayer({
				id: 'canvas-layer',
				type: 'raster',
				source: 'canvas-source'
			});
		});

	});
</script>
<!--
<canvas id="mapcanvas"></canvas>
-->
<div id="map" />

<style> 

#map {
	width: 100vw;
    height: 50vh;
	background-color: #f8f8f8;
}
</style>
