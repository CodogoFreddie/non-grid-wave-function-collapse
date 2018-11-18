import React from "react";
import * as R from "ramda";

const RuleDisplay = ({
	center,
	outers,
	update,
	stateColors,
	sampleRadius,
	paintRadius,
}) => (
	<svg width={100} height={100} viewport="0 0 100 100">
		{outers.map((outer, i, { length }) => (
			<circle
				cx={50 + Math.cos((Math.PI * 2 * i) / length) * 30}
				cy={50 + Math.sin((Math.PI * 2 * i) / length) * 30}
				r={sampleRadius}
				fill={stateColors[outer]}
			/>
		))}
		<circle cx={50} cy={50} r={paintRadius} fill={stateColors[center]} />
	</svg>
);

const Rules = ({
	stateColors,
	numberOfStates,
	n,
	rules,
	setRules,
	sampleRadius,
	paintRadius,
}) => (
	<div>
		{rules.map(({ center, outers }) => (
			<RuleDisplay
				center={center}
				outers={outers}
				stateColors={stateColors}
				sampleRadius={sampleRadius}
				paintRadius={paintRadius}
			/>
		))}

		<button
			onClick={() =>
				setRules([
					...rules,
					{
						center: 0,
						outers: R.times(i => i % numberOfStates, n),
					},
				])
			}
		>
			Add New Rule
		</button>
	</div>
);

export default Rules;
