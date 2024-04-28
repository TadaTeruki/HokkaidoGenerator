<script lang="ts">
	import type { MapData } from '$lib/map';
	import { generateMapView, getDataset, initWasm } from '$lib/view';
	import { toKana } from 'wanakana';
	import { onMount } from 'svelte';
	import Cityinfo from '../components/cityinfo.svelte';
	import Introduction from '../components/introduction.svelte';

	let mapData: MapData | null = null;

	let cityName: [string, string] = ['', ''];
	let address: string = '';
	let population: string = '';
	let isLoading = false;

	const initialSeed = -1;
	let dataset: string = '';
	let seed: number | undefined = undefined;
	let isInitial = false;
	let presentationMode = false;

	function getSeed() {
		const urlParams = new URLSearchParams(location.search);
		const seedParam = urlParams.get('seed');
		return seedParam ? parseInt(seedParam) : undefined;
	}

	// onload
	onMount(async () => {
		await initWasm();
		dataset = await getDataset();
		seed = getSeed();
		if (seed === undefined) {
			isInitial = true;
			return;
		}
		const urlParams = new URLSearchParams(location.search);
		presentationMode = urlParams.get('presentation') === 'true';

		generateMap();
	});

	function generateMap() {
		isLoading = true;
		setTimeout(function () {
			if (seed === undefined) {
				return;
			}
			mapData = generateMapView(seed, dataset);
			isLoading = false;
			isInitial = false;
			history.replaceState(
				null,
				'',
				`/?seed=${seed}${presentationMode ? '&presentation=true' : ''}`
			);

			setMap(mapData);
		}, 150);
	}

	function setMap(mapData: MapData) {
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
	}

	async function newMap() {
		seed = Math.floor(Math.random() * 1000000) + 1 + initialSeed;
		await generateMap();
	}

	async function resetPage() {
		isInitial = true;
		history.replaceState(null, '', '/');
		location.reload();
	}
</script>

<div id="map" />

<div id="right">
	<header id="header">
		<a href="/" on:click={resetPage}> Hokkaido Generator 北海道ジェネレータ </a> |
		<a href="https://github.com/TadaTeruki/HokkaidoGenerator">GitHub</a>
	</header>
	<div id="control">
		{#if isInitial}
			<Introduction />
		{:else}
			<Cityinfo {cityName} {address} {population} />
		{/if}

		<button on:click={newMap} id="generateButton" disabled={isLoading}>
			{#if isLoading}
				loading...
			{:else}
				新しく生成
			{/if}
		</button>
		{#if presentationMode}
			<img src="/QR.png" alt="共有" id="qr" />
		{/if}
	</div>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Zen+Kaku+Gothic+New?weight=500&display=swap');

	:global(body) {
		margin: 0;
		display: flex;
		flex-direction: row;
		align-items: center;
		font-family: 'Zen Kaku Gothic New', sans-serif;
		width: 100vw;
		height: 100vh;
		overflow: hidden;
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

	/* responsive for mobile */
	/* if the screen width is less than its height, it is considered as mobile */
	@media (max-aspect-ratio: 1/1) {
		:global(body) {
			flex-direction: column;
		}

		#map {
			width: 100%;
			height: 50%;
		}

		#right {
			width: 100%;
			height: 50%;
		}
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

	#qr {
		width: 18rem;
		height: 18rem;
		margin-top: 1rem;
	}
</style>
