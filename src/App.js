import React, { useState, useEffect } from "react";
import * as R from "ramda";
import { adjustHue } from "polished";

import Config from "./components/Config";
import Rules from "./components/Rules";
import { createMedium } from "./engine/Main";

const App = () => {
	const [r, setR] = useState(5);
	const [n, setN] = useState(3);
	const [sampleRadius, setSampleRadius] = useState(10);
	const [paintRadius, setPaintRadius] = useState(20);
	const [a, setA] = useState(100);
	const [b, setB] = useState(100);
	const [numberOfStates, setNumberOfStates] = useState(3);
	const [stateColors, setStateColors] = useState([]);
	const [rules, setRules] = useState([]);

	useEffect(
		() => {
			setStateColors(
				R.times(
					i => adjustHue((360 * i) / numberOfStates, "#f00"),
					numberOfStates,
				),
			);
		},
		[numberOfStates],
	);

	useEffect(
		() => {
			const savedRules =
				localStorage[`savedRules_${numberOfStates}_${n}`];

			if (savedRules) {
				setRules(JSON.parse(savedRules));
			} else {
				setRules([]);
			}
		},
		[numberOfStates, n],
	);

	useEffect(
		() => {
			localStorage[`savedRules_${numberOfStates}_${n}`] = JSON.stringify(
				rules,
			);
		},
		[rules, n],
	);

	return (
		<div style={{ display: "flex", flexDirection: "column" }}>
			<Config
				{...{
					r,
					setR,
					n,
					setN,
					sampleRadius,
					setSampleRadius,
					paintRadius,
					setPaintRadius,
					a,
					setA,
					b,
					setB,
					numberOfStates,
					setNumberOfStates,
					stateColors,
				}}
			/>
			<Rules
				{...{
					numberOfStates,
					rules,
					setRules,
					n,
					r,
					stateColors,
					sampleRadius,
					paintRadius,
				}}
			/>

			{JSON.stringify({ paintRadius, sampleRadius, rules })}
		</div>
	);
};

export default App;
