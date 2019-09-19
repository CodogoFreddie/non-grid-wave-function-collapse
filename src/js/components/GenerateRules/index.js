import React from "react";

import { generate_rules } from "../../../rust/src/lib";
import { cleanConfig } from "../../util";

const GenerateRules = ({ config, rules, onChange }) => {
	const canvasRefTraining = React.useRef();
	const canvasRefSampling = React.useRef();
	const [imageFile, setImageFile] = React.useState(null);

	React.useEffect(() => {
		if (!imageFile) {
			return;
		}

		var canvas = canvasRefTraining.current;
		var context = canvas.getContext("2d");
		var imgPath = imageFile;
		var reader = new FileReader();

		reader.onload = function(evt) {
			const img = new Image();
			img.onload = () => {
				context.drawImage(img, 0, 0);
			};

			img.src = evt.target.result;
		};

		reader.readAsDataURL(imageFile);
	}, [imageFile, canvasRefTraining.current]);

	const generateRules = React.useCallback(() => {
		const rules = generate_rules(
			cleanConfig(config),
			canvasRefTraining.current.getContext("2d"),
			canvasRefSampling.current.getContext("2d"),
		);

		console.log(rules);
		onChange(rules);
	}, [config, canvasRefSampling.current]);

	return (
		<div>
			<form onSubmit={e => e.preventDefault()}>
				<label htmlFor="upload-image">Upload training image</label>
				<input
					type="file"
					accept="image/*"
					title="Upload Image"
					id="upload-image"
					onChange={e => {
						setImageFile(e.target.files[0]);
					}}
				/>
			</form>
			<canvas
				ref={canvasRefTraining}
				id="training-image"
				height="256"
				width="256"
			></canvas>
			<button onClick={generateRules}>begin</button>
			<canvas
				ref={canvasRefSampling}
				id="sampling-area"
				height="256"
				width="256"
			></canvas>
		</div>
	);
};

export default GenerateRules;
