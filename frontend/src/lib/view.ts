import init from '$lib/engine/hokkaido_generator';
import { MapData } from '$lib/map';
import maplibre, { type StyleSpecification } from 'maplibre-gl';

export async function initWasm() {
	await init();
}

export async function getDataset() {
	return await fetch('/dataset/placenames.csv').then((response) => response.text());
}

export function generateMapView(seed: number, dataset: string) {
	const width = 700;
	const height = 700;
	const mapData = new MapData(seed, width / height, width, height, dataset);

	const visual = document.createElement('canvas');
	visual.width = width;
	visual.height = height;
	mapData.drawVisual(visual);

	const heightmap = document.createElement('canvas');
	heightmap.width = width;
	heightmap.height = height;
	mapData.drawHeightmap(heightmap);

	let highwayFeature = [] as GeoJSON.Feature[];
	let streetFeature = [] as GeoJSON.Feature[];

	const bound_x = mapData.map.bound_max().x - mapData.map.bound_min().x;
	const mapXtoProportion = (x: number) => (x + 0.5 - mapData.map.bound_min().x) / bound_x;
	const bound_y = mapData.map.bound_max().y - mapData.map.bound_min().y;
	const mapYtoProportion = (y: number) => (-y + 0.5 - mapData.map.bound_min().y) / bound_y;

	const scale = 1.4;

	mapData.map.network_paths().map((path) => {
		const node1 = path.node1();
		const node2 = path.node2();

		const feature = {
			type: 'Feature',
			geometry: {
				type: 'LineString',
				coordinates: [
					[mapXtoProportion(node1.site().x) * scale, mapYtoProportion(node1.site().y) * scale],
					[mapXtoProportion(node2.site().x) * scale, mapYtoProportion(node2.site().y) * scale]
				]
			}
		} as GeoJSON.Feature;

		if (path.stage() === 0) {
			highwayFeature.push(feature);
		} else {
			streetFeature.push(feature);
		}
	});

	const imageBounds = [0, 0, 1, 1] as [number, number, number, number];

	const mapStyle: StyleSpecification = {
		version: 8,
		glyphs: 'https://fonts.openmaptiles.org/{fontstack}/{range}.pbf',
		sources: {
			visual: {
				type: 'raster',
				tiles: [visual.toDataURL('image/png')],
				tileSize: 256,
				maxzoom: 8,
				minzoom: 8,
				bounds: imageBounds
			},
			heightmap: {
				type: 'raster-dem',
				tiles: [heightmap.toDataURL('image/png')],
				tileSize: 256,
				maxzoom: 8,
				minzoom: 8,
				bounds: imageBounds
			},
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
		terrain: {
			source: 'heightmap',
			exaggeration: 0.004
		},

		layers: [
			{
				id: 'canvas-layer',
				type: 'raster',
				source: 'visual'
			},
			{
				id: 'street',
				type: 'line',
				source: 'streetPath',
				paint: {
					'line-color': '#666',
					'line-width': 0.5
				}
			},
			{
				id: 'highway',
				type: 'line',
				source: 'highwayPath',
				paint: {
					'line-color': '#333',
					'line-width': 1.5
				}
			}
		]
	};

	const originSite = mapData.map.get_origin_site();
	const originCoords = [
		mapXtoProportion(originSite.x) * scale,
		mapYtoProportion(originSite.y) * scale
	] as [number, number];

	const mapElement = document.getElementById('map');
	if (mapElement) {
		mapElement.innerHTML = '';
	}

	const maplibreMap = new maplibre.Map({
		container: 'map',
		zoom: mapData.map.get_population() > 20000 ? 10.5 : 11,
		center: originCoords,
		style: mapStyle,
		attributionControl: false,
		renderWorldCopies: false,
		pitch: 40,
		maxPitch: 85,
		bearing: (mapData.map.get_initial_angle() / Math.PI) * 180 + 45 * (seed % 2 ? 1 : -1),
		antialias: false,
		preserveDrawingBuffer: true
	});

	return [mapData, maplibreMap];
}
