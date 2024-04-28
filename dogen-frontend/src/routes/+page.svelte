<script lang="ts">
	import type { MapData } from '$lib/map';
	import { generateMapView } from '$lib/view';
	import { toKana } from 'wanakana';
	import { onMount } from 'svelte';
	import Cityinfo from '../components/cityinfo.svelte';
	import Introduction from '../components/introduction.svelte';

	let mapData: MapData | null = null;

	let cityName: [string, string] = ['', ''];
	let address: string = '';
	let population: string = '';

	const initialSeed = 0;
	let isInitial = true;

	// onload
	onMount(async () => {
		const urlParams = new URLSearchParams(window.location.search);
		const seedParam = urlParams.get('seed');
		if (seedParam) {
			isInitial = false;
			const seed = parseInt(seedParam);
			generateMap(seed);
		} else {
			isInitial = true;
			generateMap(initialSeed);
		}
	});

	async function generateMap(seed: number) {
		mapData = await generateMapView(seed);
		setMap(mapData, seed);
	}

	function setMap(mapData: MapData, seed: number) {
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

		population = '市街人口: ' + mapData.map.get_population().toLocaleString() + '人';

		history.replaceState(null, '', `/?seed=${seed}`);
	}

	async function newMap() {
		const seed = Math.floor(Math.random() * 1000000) + 1 + initialSeed;
		await generateMap(seed);
		isInitial = false;
	}
</script>

<div id="map" />

<div id="right">
	<header id="header">Hokkaido Generator -北海道ジェネレータ-</header>
	<div id="control">
		{#if isInitial}
			<Introduction />
		{:else}
			<Cityinfo {cityName} {address} {population} />
		{/if}
		<button on:click={newMap} id="generateButton">新しく生成</button>
	</div>
	<footer id="footer">
		<a href="https://github.com/TadaTeruki/HokkaidoGenerator">GitHub</a>
	</footer>
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

	#right {
		width: 50%;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	#control {
		flex: 1;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		text-align: center;
		gap: 0.5rem;
	}

	#header {
		font-size: 1rem;
		color: #888;
		text-align: center;
		margin: 0.5rem auto;
	}

	#header:hover {
		color: #aaa;
	}

	#footer {
		font-size: 1rem;
		color: #888;
		text-align: center;
		margin: 1.5rem auto;
	}

	a {
		color: #888;
		border-bottom: 0.5px solid #888;
		text-decoration: none;
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
