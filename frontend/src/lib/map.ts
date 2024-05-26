import { toKana } from "wanakana";
import { MapFactors, MapView } from "./view";

export class PlaceName {
    cityName: [string, string] = ['', ''];
    address: string = '';
    population: string = '';
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
}

export function createMap(placenameDataset: string, view3D: boolean, nightMode: boolean, presetSeed: number | undefined): MapSet {
    const seed = presetSeed ? presetSeed : Math.floor(Math.random() * 1000000) + 1;
    const mapFactors = new MapFactors(seed, placenameDataset, nightMode);
    history.replaceState(
        null,
        '',
        `/?seed=${seed}`
    );

    const mapData = mapFactors.mapData;
    const cityName: [string, string] = [
        mapData.map.get_nameset().city_name().name(),
        toKana(mapData.map.get_nameset().city_name().reading())
    ];

    const gov = mapData.map.get_nameset().government().name();
    let county = '';
    if (gov !== '市') {
        county = mapData.map.get_nameset().county_name().name() + '郡 ';
    }

    const address =
        mapData.map.get_nameset().subprefecture_name().name() +
        mapData.map.get_nameset().subprefecture_postfix().name() +
        ' ' +
        county +
        cityName[0] +
        mapData.map.get_nameset().government().name();

    const population = '市街人口: ' + mapData.map.get_population().toLocaleString() + '人';

    const mapView = new MapView(mapFactors, view3D, nightMode);

    return new MapSet(mapFactors, mapView, { cityName, address, population }); 

}
