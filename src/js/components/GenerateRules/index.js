import React from "react";

const GenerateRules = () => {
	const canvasRef = React.useRef();
	const [imageFile, setImageFile] = React.useState(null);

	React.useEffect(() => {
		if (!imageFile) {
			return;
		}

		var canvas = canvasRef.current;
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
	}, [imageFile, canvasRef.current]);

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
				ref={canvasRef}
				id="training-image"
				height="256"
				width="256"
			></canvas>
		</div>
	);
};

export default GenerateRules;
