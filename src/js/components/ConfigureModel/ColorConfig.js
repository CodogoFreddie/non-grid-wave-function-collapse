import React from "react";
import * as R from "ramda";

const ColorConfig = ({ id, value, onChange }) => {
	return (
		<fieldset>
			<legend>{value.name}</legend>

			<label for={`${id}-name`}>Name</label>
			<input
				id={`${id}-name`}
				type="text"
				value={value.name}
				onChange={e => {
					onChange(
						R.assocPath(["states", id, "name"], e.target.value),
					);
				}}
			/>

			<br />

			<label for={`${id}-color`}>Color</label>
			<input
				id={`${id}-color`}
				type="color"
				value={value.color}
				onChange={e => {
					onChange(
						R.assocPath(["states", id, "color"], e.target.value),
					);
				}}
			/>

			<br />

			<button
				onClick={() => {
					onChange(R.dissocPath(["states", id]));
				}}
			>
				Delete
			</button>
		</fieldset>
	);
};

export default ColorConfig;
