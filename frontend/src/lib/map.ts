import { toKana } from 'wanakana';
import { MapFactors, MapView } from './view';

export class PlaceName {
	cityNameKanji: string = '';
	cityNameKana: string = '';
	cityNameRome: string = '';
	addressJa: string = '';
	addressEn: string = '';
	populationJa: string = '';
	populationEn: string = '';
}

export class MapSet {
	factors: MapFactors;
	view: MapView;
	placeName: PlaceName;

	constructor(factors: MapFactors, view: MapView, placeName: PlaceName) {
		this.factors = factors;
		this.view = view;
		this.placeName = placeName;
	}

	updateView(view3D: boolean, darkMode: boolean) {
		this.factors.updateDarkMode(darkMode);
		this.view.updateFactors(this.factors, view3D, darkMode);
	}

	screenShot() {
		const canvas = this.view.maplibreMap?.getCanvas() as HTMLCanvasElement;
		const croppedCanvas = document.createElement('canvas');

		const croppedWH = canvas.width > canvas.height ? canvas.height : canvas.width;
		croppedCanvas.width = croppedWH;
		croppedCanvas.height = croppedWH;
		const croppedCtx = croppedCanvas.getContext('2d') as CanvasRenderingContext2D;
		// get background color
		const upperBg = getComputedStyle(document.documentElement).getPropertyValue('--map-upper-bg');
		const lowerBg = getComputedStyle(document.documentElement).getPropertyValue('--map-lower-bg');
		// draw gradient background
		const gradient = croppedCtx.createLinearGradient(0, 0, 0, croppedWH);
		gradient.addColorStop(0, upperBg);
		gradient.addColorStop(0.3, lowerBg);
		croppedCtx.fillStyle = gradient;
		croppedCtx.fillRect(0, 0, croppedWH, croppedWH);

		croppedCtx.drawImage(
			canvas,
			Math.min(0, (canvas.width - croppedWH) / 2),
			Math.min(0, (canvas.height - croppedWH) / 2),
			croppedWH,
			croppedWH,
			0,
			0,
			croppedWH,
			croppedWH
		);

		const image = croppedCanvas.toDataURL('image/png');
		return image;
	}
}

function format(str: string) {
	const base = str.charAt(0).toUpperCase() + str.slice(1);
	const result = base
		.replace('aa', 'a')
		.replace('ii', 'i')
		.replace('uu', 'u')
		.replace('ee', 'e')
		.replace('oo', 'o');
	return result;
}

export function createMap(
	placenameDataset: string,
	view3D: boolean,
	darkMode: boolean,
	seed: number
): MapSet {
	const mapFactors = new MapFactors(seed, placenameDataset, darkMode);

	const mapData = mapFactors.mapData;

	const cityNameKanji = mapData.map.get_nameset().city_name().name();
	const cityNameKana = toKana(mapData.map.get_nameset().city_name().reading());

	const govJa = mapData.map.get_nameset().government().name();

	let countyJa = '';
	let countyEn = '';
	if (govJa !== '市') {
		countyJa = mapData.map.get_nameset().county_name().name() + '郡 ';
		countyEn = mapData.map.get_nameset().county_name().reading() + ' District';
	}

	let govEn = '';
	if (govJa === '市') {
		govEn = 'City';
	}
	if (govJa === '町') {
		govEn = 'Town';
	}
	if (govJa === '村') {
		govEn = 'Village';
	}

	const cityNameRome = format(mapData.map.get_nameset().city_name().reading());

	const addressJa =
		mapData.map.get_nameset().subprefecture_name().name() +
		mapData.map.get_nameset().subprefecture_postfix().name() +
		' ' +
		countyJa +
		cityNameKanji +
		mapData.map.get_nameset().government().name();

	const addressEn =
		format(cityNameRome) +
		' ' +
		govEn +
		', ' +
		(function () {
			if (countyEn) return format(countyEn) + ', ';
			else return ' ';
		})() +
		format(mapData.map.get_nameset().subprefecture_name().reading()) +
		' Subprefecture ';

	const populationJa = '市街人口: ' + mapData.map.get_population().toLocaleString() + '人';
	const populationEn = 'Population: ' + mapData.map.get_population().toLocaleString();

	const mapView = new MapView(mapFactors, view3D, darkMode);

	return new MapSet(mapFactors, mapView, {
		cityNameKanji: cityNameKanji,
		cityNameKana: cityNameKana,
		cityNameRome: cityNameRome,
		addressJa: addressJa,
		addressEn: addressEn,
		populationJa: populationJa,
		populationEn: populationEn
	});
}
