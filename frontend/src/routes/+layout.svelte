<script>
	import { _ } from 'svelte-i18n';
</script>

<div id="map" />
<div id="right">
	<header id="header">
		<a href="/"> {$_('nav-title')} </a> |
		<a href="https://github.com/TadaTeruki/HokkaidoGenerator">GitHub</a> |
		<a href="https://peruki.dev">peruki.dev</a>
	</header>
	<slot />
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

	@media (prefers-color-scheme: light) {
		:global(:root) {
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

	@media (prefers-color-scheme: dark) {
		:global(:root) {
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

	#header {
		font-size: 1rem;
		color: var(--sub-text);
		text-align: center;
		margin: 0.5rem auto;
	}

	#header:hover {
		color: var(--sub-text-hover);
	}

	:global(a) {
		color: var(--sub-text);
		border-bottom: 0.5px solid var(--sub-text);
		text-decoration: none;
	}
</style>
