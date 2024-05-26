import { NetworkPath } from '$lib/engine/hokkaido_generator';
import maplibre, { type StyleSpecification } from 'maplibre-gl';
import { MapData } from './data';

export class MapFactors {
	visual: HTMLCanvasElement;
	heightmap: HTMLCanvasElement;
	highwayFeature: GeoJSON.Feature[];
	streetFeature: GeoJSON.Feature[];
	mapData: MapData;
	originCoords: [number, number];
	seed: number;

	constructor(seed: number, dataset: string, darkMode: boolean) {
		this.seed = seed;
		const width = 700;
		const height = 700;
		this.mapData = new MapData(seed, width / height, width, height, dataset);

		this.visual = document.createElement('canvas');
		this.visual.width = width;
		this.visual.height = height;
		this.mapData.drawVisual(this.visual, darkMode);

		this.heightmap = document.createElement('canvas');
		this.heightmap.width = width;
		this.heightmap.height = height;
		this.mapData.drawHeightmap(this.heightmap);

		this.highwayFeature = [] as GeoJSON.Feature[];
		this.streetFeature = [] as GeoJSON.Feature[];

		const bound_x = this.mapData.map.bound_max().x - this.mapData.map.bound_min().x;
		const mapXtoProportion = (x: number) => (x + 0.5 - this.mapData.map.bound_min().x) / bound_x;
		const bound_y = this.mapData.map.bound_max().y - this.mapData.map.bound_min().y;
		const mapYtoProportion = (y: number) => (-y + 0.5 - this.mapData.map.bound_min().y) / bound_y;

		const scale = 1.4;

		this.mapData.map.network_paths().map((path: NetworkPath) => {
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
				this.highwayFeature.push(feature);
			} else {
				this.streetFeature.push(feature);
			}
		});

		const originSite = this.mapData.map.get_origin_site();
		this.originCoords = [
			mapXtoProportion(originSite.x) * scale,
			mapYtoProportion(originSite.y) * scale
		] as [number, number];
	}

	visualURL() {
		return this.visual.toDataURL();
	}

	heightmapURL() {
		return this.heightmap.toDataURL();
	}

	updateNightMode(darkMode: boolean) {
		this.mapData.drawVisual(this.visual, darkMode);
	}
}

export class MapView {
	maplibreMap: maplibre.Map | undefined;
	factors: MapFactors | undefined;

	constructor(factors: MapFactors, view3D: boolean, darkMode: boolean) {
		this.updateFactors(factors, view3D, darkMode);
	}

	updateStyle(view3D: boolean, darkMode: boolean) {
		if (!this.factors) return;
		const mapStyle = setupMapStyle(this.factors, view3D, darkMode);
		this.maplibreMap?.setStyle(mapStyle);
	}

	updateFactors(factors: MapFactors, view3D: boolean, darkMode: boolean) {
		const initial = !this.factors;
		this.factors = factors;

		const mapStyle = setupMapStyle(this.factors, view3D, darkMode);

		if (initial) {
			const mapElement = document.getElementById('map');
			if (mapElement) {
				mapElement.innerHTML = '';
			}

			this.maplibreMap = new maplibre.Map({
				container: 'map',
				zoom: factors.mapData.map.get_population() > 20000 ? 10 : 10.5,
				center: factors.originCoords,
				style: mapStyle,
				attributionControl: false,
				renderWorldCopies: false,
				pitch: 40,
				maxPitch: 85,
				bearing:
					(factors.mapData.map.get_initial_angle() / Math.PI) * 180 +
					45 * (factors.seed % 2 ? 1 : -1),
				antialias: false,
				preserveDrawingBuffer: true
			});
		} else {
			this.maplibreMap?.setStyle(mapStyle);
		}
	}
}

function setupMapStyle(
	factors: MapFactors,
	view3D: boolean,
	darkMode: boolean
): StyleSpecification {
	const imageBounds = [0, 0, 1, 1] as [number, number, number, number];

	const mapStyle: StyleSpecification = {
		version: 8,
		glyphs: 'https://fonts.openmaptiles.org/{fontstack}/{range}.pbf',
		sources: {
			visual: {
				type: 'raster',
				tiles: [factors.visualURL()],
				tileSize: 256,
				maxzoom: 8,
				minzoom: 8,
				bounds: imageBounds
			},
			heightmap: {
				type: 'raster-dem',
				tiles: [factors.heightmapURL()],
				tileSize: 256,
				maxzoom: 8,
				minzoom: 8,
				bounds: imageBounds
			},
			highwayPath: {
				type: 'geojson',
				data: {
					type: 'FeatureCollection',
					features: factors.highwayFeature
				}
			},
			streetPath: {
				type: 'geojson',
				data: {
					type: 'FeatureCollection',
					features: factors.streetFeature
				}
			}
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
				paint: darkMode
					? {
							'line-color': '#ddccaa',
							'line-width': 2.0,
							'line-dasharray': [1, 8]
						}
					: {
							'line-color': '#666',
							'line-width': 0.5
						}
			},
			{
				id: 'highway',
				type: 'line',
				source: 'highwayPath',
				paint: darkMode
					? {
							'line-color': '#ffedd5',
							'line-width': 2.0,
							'line-gap-width': 1.0,
							'line-dasharray': [1, 1]
						}
					: {
							'line-color': '#333',
							'line-width': 1.5
						}
			}
		]
	};

	if (view3D) {
		mapStyle.terrain = {
			source: 'heightmap',
			exaggeration: 0.004
		};
	}

	return mapStyle;
}
