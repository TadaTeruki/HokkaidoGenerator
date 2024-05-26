import { setupI18n } from "$lib/translation";

import { MapFactors, MapView, getDataset, initWasm } from '$lib/view';
import { toKana } from 'wanakana';
import { onMount } from 'svelte';
import { _ } from 'svelte-i18n'
import { PageConfig } from '$lib/config';

let pageConfig: PageConfig;
let mapFactors: MapFactors;
let mapView: MapView;

let cityName: [string, string] = ['', ''];
let address: string = '';
let population: string = '';
let isLoading = false;

const initialSeed = -1;
let dataset: string = '';
let seed: number | undefined = undefined;
let isInitial = false;
let presentationMode = false;

	/*

	function preferedNightMode() {
		const localNightMode = localStorage.getItem('nightMode');
		if (localNightMode !== null) {
			return localNightMode === 'true';
		}
		return window.matchMedia('(prefers-color-scheme: dark)').matches;
	}
	
	function onWindowLoad() {
		pageConfig = new PageConfig(false, preferedNightMode());
	}


	onMount(async () => {
		await initWasm();
		dataset = await getDataset();

		const urlParams = new URLSearchParams(location.search);
		presentationMode = urlParams.get('presentation') === 'true';
		const seedParam = urlParams.get('seed');
		seed = seedParam ? parseInt(seedParam) : undefined;
		if (seed === undefined) {
			isInitial = true;
			return;
		}

		generateMap();
	});

	async function newMap() {
		seed = Math.floor(Math.random() * 1000000) + 1 + initialSeed;
		generateMap();
	}

	async function toTopPage() {
		isInitial = true;
		history.replaceState(null, '', '/');
		location.reload();
	}
	/*
	$:if (document && pageConfig?.nightMode) {
		document.documentElement.classList.add('dark');
		document.documentElement.classList.remove('light');
		localStorage.setItem('nightMode', 'true');
	} else {
		document.documentElement.classList.remove('dark');
		document.documentElement.classList.add('light');
		localStorage.setItem('nightMode', 'false');
	}
	*/
	function onToggleNightMode(next:boolean) {
		pageConfig.nightMode = next;
		if (next) {
			document.documentElement.classList.add('dark');
			document.documentElement.classList.remove('light');
			localStorage.setItem('nightMode', 'true');
		} else {
			document.documentElement.classList.remove('dark');
			document.documentElement.classList.add('light');
			localStorage.setItem('nightMode', 'false');
		}
		mapFactors.updateNightMode(pageConfig.nightMode);
		mapView.updateFactors(mapFactors, pageConfig.view3D, pageConfig.nightMode);
	}

	function onToggle3D(next:boolean) {
		pageConfig.view3D = next;
		mapView.updateFactors(mapFactors, pageConfig.view3D, pageConfig.nightMode);
	}

	$: {
		if (mapFactors) {
			mapFactors.updateNightMode(pageConfig.nightMode);
			mapView.updateFactors(mapFactors, pageConfig.view3D, pageConfig.nightMode);
		}
	}
	*/

/** @type {import('./$types').PageLoad} */
export function load() {
    setupI18n('en');
}



