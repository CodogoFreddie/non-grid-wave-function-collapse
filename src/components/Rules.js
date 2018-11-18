import React from "react";
import * as R from "ramda";

const RuleDisplay = ({
	center,
	outers,
	update,
	stateColors,
	sampleRadius,
	paintRadius,
	onClickCenter,
	onClickOuter,
	r,
}) => (
	<svg width={100} height={100} viewport="0 0 100 100">
		{outers.map((outer, i, { length }) => (
			<circle
				cx={50 + Math.cos((Math.PI * 2 * i) / length) * r}
				cy={50 + Math.sin((Math.PI * 2 * i) / length) * r}
				r={sampleRadius}
				fill={stateColors[outer]}
				onClick={onClickOuter(i)}
			/>
		))}
		<circle
			cx={50}
			cy={50}
			r={paintRadius}
			fill={stateColors[center]}
			onClick={onClickCenter}
		/>
	</svg>
);

const Rules = ({
	stateColors,
	numberOfStates,
	n,
	r,
	rules,
	setRules,
	sampleRadius,
	paintRadius,
}) => (
	<div>
		{rules.map(({ center, outers }, i) => (
			<RuleDisplay
				center={center}
				outers={outers}
				stateColors={stateColors}
				sampleRadius={sampleRadius}
				paintRadius={paintRadius}
				r={r}
				onClickCenter={() =>
					setRules(
						R.over(
							R.lensPath([i, "center"]),
							R.pipe(
								R.tap(console.log),
								R.inc,
								R.tap(console.log),
								R.modulo(R.__, numberOfStates),
								R.tap(console.log),
							),
						),
					)
				}
				onClickOuter={outerIndex => () =>
					setRules(
						R.over(
							R.lensPath([i, "outers", outerIndex]),
							R.pipe(
								R.inc,
								R.modulo(R.__, numberOfStates),
							),
						),
					)}
			/>
		))}

		<button
			onClick={() =>
				setRules(
					R.append({
						center: 0,
						outers: R.times(i => i % numberOfStates, n),
					}),
				)
			}
		>
			Add New Rule
		</button>

		<button
			onClick={() => {
				setRules(R.init);
			}}
		>
			Delete Rule
		</button>
	</div>
);

export default Rules;
