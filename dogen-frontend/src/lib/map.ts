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
	elevationBuffer: ElevationBuffer;
	imageWidth: number;
	imageHeight: number;

	constructor(seed: number, x_expand_prop: number, imageWidth: number, imageHeight: number) {
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
		this.elevationBuffer = new ElevationBuffer(this.map, imageWidth, imageHeight);
		this.imageWidth = imageWidth;
		this.imageHeight = imageHeight;
	}

	createImage(colormap: Colormap, fadeRadius: number) {
		const imageData = new ImageData(this.imageWidth, this.imageHeight);
		for (let iy = 0; iy < this.imageHeight; iy++) {
			for (let ix = 0; ix < this.imageWidth; ix++) {
				const elevation = this.elevationBuffer.get_elevation(ix, iy);
				const color = colormap.getColor(elevation);
				const index = (iy * this.imageWidth + ix) * 4;
				imageData.data[index] = color[0];
				imageData.data[index + 1] = color[1];
				imageData.data[index + 2] = color[2];

				const alphaSeed = Math.min(
					Math.min(Math.min(ix, this.imageWidth - ix), Math.min(iy, this.imageHeight - iy)) /
						fadeRadius,
					1.0
				);

				imageData.data[index + 3] = 255 * Math.pow(alphaSeed, 0.8);
			}
		}

		return imageData;
	}

	drawVisual(canvas: HTMLCanvasElement) {
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

		const imageData = this.createImage(colormap, 8);
		ctx.putImageData(imageData, 0, 0);
	}

	drawHeightmap(canvas: HTMLCanvasElement) {
		const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
		const colormap = new Colormap(
			[
				[0, 0, 0],
				[255, 255, 255]
			],
			[0.0, 100.0]
		);

		const imageData = this.createImage(colormap, 0);
		ctx.putImageData(imageData, 0, 0);
	}
}
