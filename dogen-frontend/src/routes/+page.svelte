<script lang="ts">
	import init from '$lib/engine/dogen_generator';
	import { MapData } from '$lib/map';
	import maplibre, { type StyleSpecification } from 'maplibre-gl';
	import { onMount } from 'svelte';

	onMount(async () => {
		await init();

		const width = 500;
		const height = 500;
		const seed = Math.floor(Math.random() * 1000);

		const mapData = new MapData(seed, width / height, width, height);

		const visual = document.createElement('canvas');
		visual.width = width;
		visual.height = height;

		mapData.drawVisual(visual);

		const heightmap = document.createElement('canvas');
		heightmap.width = width;
		heightmap.height = height;

		mapData.drawHeightmap(heightmap);

		console.log('loading');

		const mapStyle: StyleSpecification = {
			version: 8,

			sources: {
				visual: {
					type: 'raster',
					tiles: [visual.toDataURL('image/jpeg')],
					bounds: [-1, -1, 1, 1],
					minzoom: 0,
					maxzoom: 0,
					tileSize: Math.max(width, height)
				},
				heightmap: {
					type: 'raster-dem',
					tiles: [heightmap.toDataURL('image/png')],
					minzoom: 0,
					maxzoom: 0,
					tileSize: Math.max(width, height),
					bounds: [-1, -1, 1, 1]
				}
			},

			terrain: {
				source: 'heightmap',
				exaggeration: 0.5
			},

			layers: [
				{
					id: 'canvas-layer',
					type: 'raster',
					source: 'visual'
				},
				{
					id: 'shadow',
					type: 'hillshade',
					source: 'heightmap',
					paint: {
						'hillshade-exaggeration': 0.03,
						'hillshade-shadow-color': '#000',
						'hillshade-highlight-color': '#fff'
					}
				}
			]
		};

		new maplibre.Map({
			container: 'map',
			zoom: 0,
			center: [0, 0],
			style: mapStyle,
			renderWorldCopies: false,
			maxPitch: 85
		});

		console.log('done');
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
