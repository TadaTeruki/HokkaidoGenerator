<script lang="ts">
	//import { MapLibre } from 'svelte-maplibre';
    import { StandardMap } from '$lib/engine/dogen_generator.js';

    class MapData {
        map: StandardMap;

        constructor() {
            this.map = new StandardMap();
        }
    }

    function drawMap() {
        const canvas = document.getElementById('canvasID') as HTMLCanvasElement;
        const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
    }

</script>

<canvas id="map"></canvas>

<style>
</style>
