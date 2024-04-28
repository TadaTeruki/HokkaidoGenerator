<script lang="ts">
	import type { MapData } from '$lib/map';
	import { generateMapView } from '$lib/view';
	import { toKana } from 'wanakana';
	import { onMount } from 'svelte';
	import Cityinfo from '../components/cityinfo.svelte';

	let mapData: MapData | null = null;

	let cityName: [string, string] = ['', ''];
	let address: string = '';
	let population: string = '';

	//onMount(reloadMap);

	async function reloadMap() {
		mapData = await generateMapView();
		cityName = [
			mapData.map.get_nameset().city_name().name(),
			toKana(mapData.map.get_nameset().city_name().reading())
		];

		const gov = mapData.map.get_nameset().government().name();
		let county = '';
		if (gov !== '市') {
			county = mapData.map.get_nameset().county_name().name() + '郡 ';
		}

		address =
			mapData.map.get_nameset().subprefecture_name().name() +
			mapData.map.get_nameset().subprefecture_postfix().name() +
			' ' +
			county +
			cityName[0] +
			mapData.map.get_nameset().government().name();
		// population with comma
		population = '市街人口: ' + mapData.map.get_population().toLocaleString() + '人';
	}
</script>

<div id="map" />

<div id="control">
	<Cityinfo {cityName} {address} {population} />
	<button on:click={reloadMap} id="generateButton">新しく生成</button>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Zen+Kaku+Gothic+New?weight=500&display=swap');

	:global(body) {
		margin: 0;
		display: flex;
		align-items: center;
		font-family: 'Zen Kaku Gothic New', sans-serif;
		width: 100vw;
		height: 100vh;
	}

	#map {
		width: 50%;
		height: 100%;
		background-color: #f0f0f0;
		display: flex;
		align-items: center;
	}

	#control {
		width: 50%;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
	}

	button {
		border: none;
		background-color: #333;
		color: #f0f0f0;
		border-radius: 0.5rem;
		padding: 0.5rem 1rem;
	}

	button:hover {
		background-color: #888;
	}

	button:active {
		animation: generating 0.4s alternate infinite;
	}

	#generateButton {
		margin-top: 1rem;
		font-weight: bold;
	}

	@keyframes generating {
		0% {
			background-color: #555;
		}
		100% {
			background-color: #888;
		}
	}
</style>
