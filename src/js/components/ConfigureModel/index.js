import React from "react";
import * as R from "ramda";

import ColorConfig from "./ColorConfig";

const ConfigureModel = ({ value, onChange, incrementStage }) => {
	return (
		<form
			onSubmit={e => {
				e.preventDefault();
			}}
		>
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
		</form>
	);
};

export default ConfigureModel;
