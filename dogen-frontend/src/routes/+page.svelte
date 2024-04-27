<script lang="ts">
	import init from '$lib/engine/dogen_generator';
	import { MapData } from '$lib/map';
	import maplibre, { type StyleSpecification } from 'maplibre-gl';
	import { onMount } from 'svelte';

	// convert bound values in range [-1, 1] to coordinates of mercator projection
	function mercator(x: number, y: number) {
		const lat = (y / Math.PI) * 180;
		const lon = (x / Math.PI) * 180;
		return [lon, lat];
	}

	onMount(async () => {
		await init();

		const width = 500;
		const height = 500;
		const seed = Math.floor(Math.random() * 10000);

		const mapData = new MapData(seed, width / height, width, height);

		const visual = document.createElement('canvas');
		visual.width = width;
		visual.height = height;
		mapData.drawVisual(visual);

		const heightmap = document.createElement('canvas');
		heightmap.width = width;
		heightmap.height = height;
		mapData.drawHeightmap(heightmap);

		const scale = 0.05;

		let highwayFeature = [] as GeoJSON.Feature[];
		let streetFeature = [] as GeoJSON.Feature[];

		const bound_x = mapData.map.bound_max().x - mapData.map.bound_min().x;
		const mapXtoProportion = (x: number) =>
			((x + 0.5 - mapData.map.bound_min().x) / bound_x) * scale;
		const bound_y = mapData.map.bound_max().y - mapData.map.bound_min().y;
		const mapYtoProportion = (y: number) =>
			((-y + 0.5 - mapData.map.bound_min().y) / bound_y) * scale;

		mapData.map.network_paths().map((path) => {
			const node1 = path.node1();
			const node2 = path.node2();

			const feature = {
				type: 'Feature',
				geometry: {
					type: 'LineString',
					coordinates: [
						mercator(mapXtoProportion(node1.site().x), mapYtoProportion(node1.site().y)),
						mercator(mapXtoProportion(node2.site().x), mapYtoProportion(node2.site().y))
					]
				}
			} as GeoJSON.Feature;

			if (path.stage() === 0) {
				highwayFeature.push(feature);
			} else {
				streetFeature.push(feature);
			}
		});

		const imageCoords = [
			mercator(0.0, scale),
			mercator(scale, scale),
			mercator(scale, 0.0),
			mercator(0.0, 0.0)
		] as [[number, number], [number, number], [number, number], [number, number]];

		const mapStyle: StyleSpecification = {
			version: 8,
			sources: {
				visual: {
					type: 'image',
					url: visual.toDataURL('image/png'),
					coordinates: imageCoords
				},
				/*
				heightmap: {
					type: 'raster-dem',
					url: heightmap.toDataURL('image/png'),
					bounds: [
						imageCoords[0][0],
						imageCoords[0][1],
						imageCoords[2][0],
						imageCoords[2][1]
					],
					tileSize: 2560
				},
				*/
				highwayPath: {
					type: 'geojson',
					data: {
						type: 'FeatureCollection',
						features: highwayFeature
					}
				},
				streetPath: {
					type: 'geojson',
					data: {
						type: 'FeatureCollection',
						features: streetFeature
					}
				}
			},
			/*
			terrain: {
				source: 'heightmap',
				exaggeration: 50
			},
			*/

			layers: [
				{
					id: 'canvas-layer',
					type: 'raster',
					source: 'visual'
				},
				/*
				{
					id: 'shadow',
					type: 'hillshade',
					source: 'heightmap',
					paint: {
						'hillshade-exaggeration': 0.03,
						'hillshade-shadow-color': '#000',
						'hillshade-highlight-color': '#fff'
					}
				},
				*/

				{
					id: 'street',
					type: 'line',
					source: 'streetPath',
					paint: {
						'line-color': '#333',
						'line-opacity': 0.45,
						'line-width': 1.0
					}
				},
				{
					id: 'highway',
					type: 'line',
					source: 'highwayPath',
					paint: {
						'line-color': '#224',
						'line-width': 1.5
					}
				}
			]
		};

		const originSite = mapData.map.get_origin_site();
		const originCoords = mercator(
			mapXtoProportion(originSite.x),
			mapYtoProportion(originSite.y)
		) as [number, number];

		new maplibre.Map({
			container: 'map',
			zoom: 8,
			center: originCoords,
			style: mapStyle,
			renderWorldCopies: false,
			maxPitch: 85,
			preserveDrawingBuffer: true,
			maxTileCacheZoomLevels: 0
		});

		console.log('done');
	});
</script>

<div id="map" />

<style>
	#map {
		width: 80vmin;
		height: 80vmin;
		background-color: #f8f8f8;
	}
</style>
