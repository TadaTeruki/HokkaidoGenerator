// place files you want to import through the `$lib` alias in this folder.

import { Colormap, colorToHex } from './color';
import { create_standard_map, type StandardMap } from './engine/dogen_generator';

function xorshift(x: number) {
	x ^= x << 13;
	x ^= x >> 17;
	x ^= x << 5;
	return x;
}

export class MapData {
	map: StandardMap;

	constructor(seed: number, x_expand_prop: number) {
		this.map = (function () {
			while (true) {
				console.log('creating map with seed', seed);
				const map = create_standard_map(seed, x_expand_prop);
				if (map) {
					return map;
				} else {
					seed += 1 + (Math.abs(xorshift(seed)) % 100);
				}
			}
		})();
	}

	propToMapBoundX(x_prop: number) {
		return this.map.bound_min().x + (this.map.bound_max().x - this.map.bound_min().x) * x_prop;
	}

	propToMapBoundY(y_prop: number) {
		return this.map.bound_min().y + (this.map.bound_max().y - this.map.bound_min().y) * y_prop;
	}

	drawTerrain(canvas: HTMLCanvasElement) {
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

		const colormap = new Colormap(
			[
				[70, 150, 200],
				[240, 240, 210],
				[190, 200, 120],
				[25, 100, 25],
				[15, 60, 15]
			],
			[0.0, 0.1, 0.15, 30.0, 50.0]
		);

		let imageData = new ImageData(canvas.width, canvas.height);
		for (let iy = 0; iy < canvas.height; iy++) {
			for (let ix = 0; ix < canvas.width; ix++) {
				const mx = this.propToMapBoundX(ix / canvas.width);
				const my = this.propToMapBoundY(iy / canvas.height);
				const elevation = this.map.get_elevation(mx, my) || 0.0;
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
