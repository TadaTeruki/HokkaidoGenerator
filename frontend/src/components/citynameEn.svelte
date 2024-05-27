<script lang="ts">
	import type { MapSet } from '$lib/map';
	export let mapSet: MapSet | undefined = undefined;
</script>

<div class="citynamebox-outer">
	<div class="citynamebox-inner">
		<span class="citykanji">
			{mapSet?.placeName.cityNameKanji}
			/
			{mapSet?.placeName.cityNameKana}</span
		>
		<div class="cityname">{mapSet?.placeName.cityNameRome}</div>
	</div>
	<br />
</div>
<div class="address">{mapSet?.placeName.addressEn}</div>
<div class="population">{mapSet?.placeName.populationEn}</div>

<style>
	.cityname {
		font-size: 3rem;
		color: var(--main-text);
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

	.citykanji {
		font-size: 1.2rem;
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
