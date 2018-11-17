import React from "react";

const Slider = ({ value, onChange, min, max, label }) => (
	<div>
		{label}:
		<input
			type="range"
			value={value}
			min={min}
			max={max}
			onChange={e => onChange(e.target.value)}
		/>
	</div>
);

const StateColors = ({ colors }) => (
	<div style={{ display: "flex", width: "100%" }}>
		{colors.map((col, i) => (
			<div style={{ backgroundColor: col, minHeight: 50, flex: 1 }}>
				{i}
			</div>
		))}
	</div>
);

const Config = ({
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
}) => (
	<div style={{ display: "flex", flexDirection: "column" }}>
		<Slider label="Radius (r)" value={r} min={3} max={50} onChange={setR} />
		<Slider
			label="Number of diameter samples (n)"
			value={n}
			min={3}
			max={10}
			onChange={setN}
		/>
		<Slider
			label="Samples (a)"
			value={a}
			min={10}
			max={500}
			onChange={setA}
		/>
		<Slider
			label="Samples (b)"
			value={b}
			min={10}
			max={500}
			onChange={setB}
		/>
		<Slider
			label="number of states"
			value={numberOfStates}
			min={2}
			max={12}
			onChange={setNumberOfStates}
		/>
		<StateColors colors={stateColors} />
	</div>
);

export default Config;
