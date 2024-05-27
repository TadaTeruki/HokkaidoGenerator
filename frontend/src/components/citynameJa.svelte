<script lang="ts">
	import type { MapSet } from '$lib/map';
	export let mapSet: MapSet | undefined = undefined;
</script>

<div class="citynamebox-outer">
	<div class="citynamebox-inner">
		<div class="citykana">{mapSet?.placeName.cityNameKana || ''}</div>
		<div class="cityname">{mapSet?.placeName.cityNameKanji || ''}</div>
	</div>
	<span class="citypostfix">{mapSet?.placeName.cityNameKanji ? '市街' : ''}</span>
	<br />
</div>
<div class="address">{mapSet?.placeName.addressJa}</div>
<div class="population">{mapSet?.placeName.populationJa}</div>

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
</style>
