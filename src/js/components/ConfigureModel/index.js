import React from "react";
import * as R from "ramda";

import { render_settings_preview } from "../../../rust/src/lib";

import ColorConfig from "./ColorConfig";
import { cleanConfig } from "../../util";

const ConfigField = ({
	value,
	onChange,
	name,
	max,
	map = x => parseInt(x, 10),
	type = "number",
	min = "1",
}) => (
	<React.Fragment>
		<label htmlFor={["settings", name].join("-")}>{name}</label>
		<input
			min={min}
			max={max}
			id={["settings", name].join("-")}
			type={type}
			value={R.path(["settings", name], value)}
			onChange={e => {
				onChange(R.assocPath(["settings", name], map(e.target.value)));
			}}
		/>
		<br />
	</React.Fragment>
);

const ConfigureModel = ({ value, onChange, incrementStage }) => {
	const configPreviewCanvas = React.useRef();
	React.useEffect(() => {
		render_settings_preview(
			cleanConfig(value),
			configPreviewCanvas.current.getContext("2d"),
		);
	}, [value]);

	return (
		<form
			onSubmit={e => {
				e.preventDefault();
			}}
		>
			<fieldset>
				<legend>settings</legend>
				<ConfigField
					value={value}
					onChange={onChange}
					name="cardinality"
				/>
				<ConfigField
					value={value}
					onChange={onChange}
					name="centerSampleWidth"
				/>
				<ConfigField
					value={value}
					onChange={onChange}
					name="outerSampleWidth"
				/>
				<ConfigField
					value={value}
					onChange={onChange}
					name="sampleSpread"
				/>

				<canvas
					ref={configPreviewCanvas}
					id="config-preview"
					height="256"
					width="256"
				></canvas>
			</fieldset>
			<fieldset>
				<legend>states</legend>
				<button
					onClick={() =>
						onChange(
							R.assocPath(
								[
									"states",
									Math.random()
										.toString(36)
										.substr(2, 5),
								],
								{ name: "", color: "#ff0000" },
							),
						)
					}
				>
					New State
				</button>

				<hr />

				{R.toPairs(value.states).map(([key, value]) => (
					<ColorConfig id={key} value={value} onChange={onChange} />
				))}
			</fieldset>

			<button onClick={incrementStage}>done</button>
		</form>
	);
};

export default ConfigureModel;
