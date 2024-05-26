<script lang="ts">
	import type { MapView } from '$lib/view';
	import { onMount } from 'svelte';

	export let cityName = ['', ''];
	export let address = '';
	export let population = '';
	export let seed = 0;
	export let mapView: MapView | undefined = undefined;

	let origin = '';
	let copiedSeed = -1;
	// image data URL
	let screenshot: string | undefined = undefined;

	onMount(() => {
		origin = window.location.origin;
	});

	function shareText(cityName: string[], population: string) {
		const shareText = `${cityName[0]}市街 (${population}) - 北海道ジェネレータ - ${origin}/?seed=${seed} #HokkaidoGenerator`;
		return shareText;
	}

	function mapScreenShot() {
		const canvas = mapView?.maplibreMap?.getCanvas() as HTMLCanvasElement;
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

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		copiedSeed = seed;
	}
</script>

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
<details class="share">
	<summary>共有する</summary>
	<p class="shareBox">
		{#if cityName[0]}
			{shareText(cityName, population)}<br />
			<button on:click={() => copyToClipboard(shareText(cityName, population))}
				>クリップボードにコピー</button
			>
			{copiedSeed === seed ? 'コピーしました' : ''}<br />
			<br />
			{#if screenshot}
				<img src={screenshot} alt="地図のスクリーンショット" />
			{:else}
				(未撮影)
			{/if}
			<br />
			<button on:click={() => (screenshot = mapScreenShot())}>地図のスクリーンショットを撮影</button
			><br />
		{/if}
	</p>
</details>

<style>
	.cityname {
		font-size: 3rem;
		color: var(--main-text);
	}

	.citypostfix {
		font-size: 2rem;
		color: var(--sub-text);
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
		color: var(--sub-text);
	}

	.address {
		font-size: 1rem;
		color: var(--sub-text);
	}

	.population {
		font-size: 1rem;
		color: var(--sub-text);
	}

	.share {
		font-size: 0.8rem;
		color: var(--sub-text);
	}

	summary {
		text-decoration: underline;
	}

	.shareBox {
		background-color: var(--sub-bg);
		padding: 1rem;
		border-radius: 0.5rem;
		transition: background-color 0.5s;
	}

	.shareBox img {
		width: 20rem;
		height: 20rem;
		object-fit: contain;
		color: #888;
		margin-top: 0.5rem;
	}

	button {
		border: none;
		background-color: transparent;
		color: #888;
		border: 0.5px solid #888;
		border-radius: 0.3rem;
		margin-top: 0.5rem;
		padding: 0.1rem 0.5rem;
	}

	button:hover {
		background-color: #00000010;
	}
</style>
