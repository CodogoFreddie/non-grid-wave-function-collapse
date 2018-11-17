import React from "react";
import * as R from "ramda";

const RuleDisplay = ({ center, outers, update, stateColors }) => (
	<svg width={100} height={100} viewport="0 0 100 100">
		<rect x={50} y={50} width={20} height={20} fill={stateColors[center]} />
		{outers.map((outer, i, { length }) => (
			<rect
				x={50 + Math.cos((Math.PI * 2 * i) / length) * 30}
				y={50 + Math.sin((Math.PI * 2 * i) / length) * 30}
				width={20}
				height={20}
				fill={stateColors[outer]}
			/>
		))}
	</svg>
);
const Rules = ({ stateColors, numberOfStates, n, rules, setRules }) => (
	<div>
		{rules.map(({ center, outers }) => (
			<RuleDisplay
				center={center}
				outers={outers}
				stateColors={stateColors}
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
