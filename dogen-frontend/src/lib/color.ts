export class Colormap {
	colors: Array<[number, number, number]>;
	weights: Array<number>;
	constructor(colors: Array<[number, number, number]>, weights: Array<number>) {
		this.colors = colors;
		this.weights = weights;
	}

	getColor(value: number) {
		let i = 0;
		while (i < this.weights.length && value > this.weights[i]) {
			i++;
		}
		if (i == 0) {
			return this.colors[0];
		}
		if (i == this.weights.length) {
			return this.colors[this.colors.length - 1];
		}
		const color1 = this.colors[i - 1];
		const color2 = this.colors[i];
		const weight1 = this.weights[i - 1];
		const weight2 = this.weights[i];
		const ratio = (value - weight1) / (weight2 - weight1);
		return [
			Math.floor(color1[0] * (1 - ratio) + color2[0] * ratio),
			Math.floor(color1[1] * (1 - ratio) + color2[1] * ratio),
			Math.floor(color1[2] * (1 - ratio) + color2[2] * ratio)
		] as [number, number, number];
	}
}

export function colorToHex(color: [number, number, number]) {
	return '#' + color.map((c) => c.toString(16).padStart(2, '0')).join('');
}
