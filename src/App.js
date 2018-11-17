import React, { useState, useEffect } from "react";
import * as R from "ramda";
import { adjustHue } from "polished";

import Config from "./components/Config";
import Rules from "./components/Rules";

const App = () => {
	const [r, setR] = useState(5);
	const [n, setN] = useState(3);
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
			setRules([]);
		},
		[numberOfStates, n],
	);

	return (
		<div style={{ display: "flex", flexDirection: "column" }}>
			<Config
				{...{
					r,
					setR,
					n,
					setN,
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
					stateColors,
				}}
			/>

			{JSON.stringify({ rules })}
		</div>
	);
};

export default App;
