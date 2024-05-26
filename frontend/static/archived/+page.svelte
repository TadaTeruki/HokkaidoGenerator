<script lang="ts">

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

	function generateMap() {
		isLoading = true;
		setTimeout(function () {
			if (seed === undefined) {
				return;
			}
			mapFactors = new MapFactors(seed, dataset, pageConfig.nightMode);
			isLoading = false;
			isInitial = false;
			history.replaceState(
				null,
				'',
				`/?seed=${seed}${presentationMode ? '&presentation=true' : ''}`
			);

			const mapData = mapFactors.mapData;
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

			mapView = new MapView(mapFactors, pageConfig.view3D, pageConfig.nightMode);
		}, 500);
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

</script>

<div id="map" class:map-daytime={!pageConfig.nightMode} class:map-night={pageConfig.nightMode} />
<div id="right">
	<header id="header">
		<a href="/" on:click={toTopPage}> {$_('nav-title')} </a> |
		<a href="https://github.com/TadaTeruki/HokkaidoGenerator">GitHub</a> |
		<a href="https://peruki.dev">peruki.dev</a>
	</header>
	<div id="control">
		{#if isInitial}
			<Introduction />
		{:else}
			<Cityinfo {cityName} {address} {population} {seed} {mapView} />
		{/if}
		<button on:click={newMap} id="generateButton" disabled={isLoading}>
			{#if isLoading}
				loading...
			{:else}
				{$_('generate-new')}
			{/if}
		</button>
		<div id="checkbox">
			<input type="checkbox" id="presentation" bind:checked={pageConfig.view3D} />
			{$_('3d-terrain')}
		</div>

		<div id="checkbox">
			<input type="checkbox" id="presentation" bind:checked={pageConfig.nightMode} />
			{$_('dark-mode')}
		</div>

		{#if presentationMode}
			<img src="/QR.png" alt="share" id="qr" />
		{/if}
	</div>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Zen+Kaku+Gothic+New:wght@500&display=swap');

	:global(:root.light) {
		--page-bg: #fafafa;
		--sub-bg: #f0f0f0;
		--map-upper-bg: #eaf0f0;
		--map-lower-bg: #f0f0f0;
		--button-bg: #333;
		--button-bg-hover: #888;
		--button-text: #f0f0f0;
		--main-text: #333;
		--sub-text: #888;
		--sub-text-hover: #aaa;
	}

	:global(:root.dark) {
		--page-bg: #202020;
		--sub-bg: #303030;
		--map-upper-bg: #202020;
		--map-lower-bg: #252040;
		--button-bg: #555;
		--button-bg-hover: #777;
		--button-text: #f0f0f0;
		--main-text: #f0f0f0;
		--sub-text: #aaa;
		--sub-text-hover: #aaa;
	}

	:global(body) {
		margin: 0;
		display: flex;
		flex-direction: row;
		align-items: center;
		font-family: 'Zen Kaku Gothic New', sans-serif;
		width: 100vw;
		height: 100vh;
		background-color: var(--page-bg);
		transition: background-color 0.25s;
	}

	#map {
		width: 50%;
		height: 100%;
		display: flex;
		align-items: center;
		background-image: linear-gradient(180deg, var(--map-upper-bg) 0%, var(--map-lower-bg) 30%);
		transition: background-color 0.25s;
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
		color: var(--sub-text);
		text-align: center;
		margin: 0.5rem auto;
	}

	#header:hover {
		color: var(--sub-text-hover);
	}

	a {
		color: var(--sub-text);
		border-bottom: 0.5px solid var(--sub-text);
		text-decoration: none;
	}

	button {
		border: none;
		background-color: var(--button-bg);
		color: var(--button-text);
		border-radius: 0.5rem;
		padding: 0.5rem 1rem;
	}

	button:hover {
		background-color: var(--button-bg-hover);
	}

	#generateButton {
		margin-top: 1rem;
		font-weight: bold;
	}

	#checkbox {
		color: var(--sub-text);
		font-size: 0.9rem;
	}

	#qr {
		width: 18rem;
		height: 18rem;
		margin-top: 1rem;
	}
</style>
