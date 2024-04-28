import init from '$lib/engine/dogen_generator';
import { MapData } from '$lib/map';
import maplibre, { type StyleSpecification } from 'maplibre-gl';

export async function generateMapView() {
	await init();
	const width = 700;
	const height = 700;

	const seed = Math.floor(Math.random() * 10000);

	const dataset = await fetch('/dataset/placenames.csv').then((response) => response.text());
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

	mapData.map.network_paths().map((path) => {
		const node1 = path.node1();
		const node2 = path.node2();

		const feature = {
			type: 'Feature',
			geometry: {
				type: 'LineString',
				coordinates: [
					[mapXtoProportion(node1.site().x), mapYtoProportion(node1.site().y)],
					[mapXtoProportion(node2.site().x), mapYtoProportion(node2.site().y)]
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
		[0.0, 1.0],
		[1.0, 1.0],
		[1.0, 0.0],
		[0.0, 0.0]
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
	const originCoords = [mapXtoProportion(originSite.x), mapYtoProportion(originSite.y)] as [
		number,
		number
	];

	const map = document.getElementById('map');
	if (map) {
		map.innerHTML = '';
	}

	new maplibre.Map({
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

	return mapData;
}
