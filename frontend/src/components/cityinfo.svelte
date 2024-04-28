<script>
	export let cityName = ['', ''];
	export let address = '';
	export let population = '';
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

<style>
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
</style>
