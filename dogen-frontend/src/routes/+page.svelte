<script lang="ts">
	import init from '$lib/engine/dogen_generator';
	import { MapData } from '$lib/map';
	import maplibre, { type StyleSpecification } from 'maplibre-gl';
	import { onMount } from 'svelte';

	onMount(async () => {
		await init();

		const width = 1000;
		const height = 1000;
		const seed = Math.floor(Math.random() * 1000);

		const mapData = new MapData(seed, width / height);

		const canvas = document.createElement('canvas');
		canvas.width = width;
		canvas.height = height;

		mapData.drawTerrain(canvas);

		const url = canvas.toDataURL('image/png');

		const mapStyle: StyleSpecification = {
			version: 8,
			sources: {
				'canvas-source': {
					type: 'image',
					url,
					coordinates: [
						[-1, 1],
						[1, 1],
						[1, -1],
						[-1, -1]
					]
				}
			},
			layers: [
				{
					id: 'canvas-layer',
					type: 'raster',
					source: 'canvas-source'
				}
			]
		};

		new maplibre.Map({
			container: 'map',
			zoom: 8,
			minZoom: 7,
			center: [0, 0],
			style: mapStyle
		});
	});
</script>

<div id="map" />

<style>
	#map {
		width: 50vmin;
		height: 50vmin;
		background-color: #f8f8f8;
	}
</style>
