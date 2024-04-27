import { Colormap } from './color';
import { create_standard_map, ElevationBuffer, type StandardMap } from './engine/dogen_generator';

function xorshift(x: number) {
	x ^= x << 13;
	x ^= x >> 17;
	x ^= x << 5;
	return x;
}

export class MapData {
	map: StandardMap;

	constructor(seed: number, x_expand_prop: number) {
		this.map = function () {
			while (true) {
				console.log('creating map with seed', seed);
				const map = create_standard_map(seed, x_expand_prop);
				if (map) {
					return map;
				} else {
					seed += 1 + (Math.abs(xorshift(seed)) % 100);
				}
			}
		}();
	}

	drawTerrain(canvas: HTMLCanvasElement) {
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

		const colormap = new Colormap(
			[
				[170, 200, 220],
				[240, 240, 210],
				[215, 230, 170],
				[25, 100, 25],
				[15, 60, 15]
			],
			[0.0, 0.1, 0.15, 40.0, 80.0]
		);

		const elevationBuffer = new ElevationBuffer(this.map, canvas.width, canvas.height);

		const imageData = new ImageData(canvas.width, canvas.height);
		for (let iy = 0; iy < canvas.height; iy++) {
			for (let ix = 0; ix < canvas.width; ix++) {
				const elevation = elevationBuffer.get_elevation(ix, iy);
				const color = colormap.getColor(elevation);
				const index = (iy * canvas.width + ix) * 4;
				imageData.data[index] = color[0];
				imageData.data[index + 1] = color[1];
				imageData.data[index + 2] = color[2];
				imageData.data[index + 3] = 255;
			}
		}

		ctx.putImageData(imageData, 0, 0);
	}
}
