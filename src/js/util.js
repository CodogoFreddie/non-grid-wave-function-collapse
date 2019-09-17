import * as R from "ramda";

export function rustifyHexColor(hexColor) {
	return R.pipe(
		R.tail,
		R.splitEvery(2),
		R.map(x => parseInt(x, 16)),
		([r, g, b]) => ({
			r,
			g,
			b,
		}),
	)(hexColor);
}

export function cleanConfig(config) {
	console.log(config);

	return R.over(
		R.lensProp("states"),
		R.map(R.over(R.lensProp("color"), rustifyHexColor)),
	)(config);
}
