<script lang="ts">
	import type { MapData } from '$lib/map';
	import { generateMapView } from '$lib/view';
	import { toKana } from 'wanakana';
	import { onMount } from 'svelte';

	let mapData: MapData | null = null;

	let cityName: [string, string] = ['', ''];
	let address: string = '';
	let population: string = '';

	onMount(reloadMap);

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
	<div class="citynamebox-outer">
		<div class="citynamebox-inner">
			<div class="citykana">{cityName[1] || ''}</div>
			<div class="cityname">{cityName[0] || ''}</div>
		</div>
		<span class="citypostfix">{cityName[0] ? '市街' : ''}</span>
		<br />
	</div>
	<div class="address">{address}</div>
	<div class="population">{population}</div>
	<button on:click={reloadMap} id="generateButton">新しく生成</button>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Zen+Kaku+Gothic+New?weight=500&display=swap');

	:global(body) {
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		font-family: 'Zen Kaku Gothic New', sans-serif;
	}

	#map {
		width: 50vw;
		height: 100vh;
		background-color: #f8f8f8;
	}

	#control {
		width: 50vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
	}

	.cityname {
		font-size: 4rem;
		color: #333;
	}

	.citypostfix {
		font-size: 2rem;
		color: #888;
	}

	.citynamebox-outer {
		display: flex;
		flex-direction: row;
		align-items: end;
	}

	.citynamebox-inner {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.citykana {
		font-size: 1.5rem;
		color: #888;
	}

	.address {
		font-size: 1rem;
		color: #888;
	}

	.population {
		font-size: 1rem;
		color: #888;
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
