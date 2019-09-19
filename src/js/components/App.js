import React from "react";
import * as R from "ramda";

import { run } from "../../rust/src/lib";

import ConfigureModel from "./ConfigureModel";
import GenerateRules from "./GenerateRules";
import RunCollapse from "./RunCollapse";

const App = () => {
	const [stage, setStage] = React.useState(0);

	const [config, setConfig] = React.useState(
		JSON.parse(
			localStorage.getItem("config") ||
				JSON.stringify({
					states: {},
					settings: {
						ruleSamples: 1000,
						paintSamples: 1000,
						cardinality: 3,
						centerSampleWidth: 30,
						outerSampleWidth: 30,
						sampleSpread: 40,
					},
				}),
		),
	);
	React.useEffect(() => {
		localStorage.setItem("config", JSON.stringify(config));
	}, [config]);

	const [rules, setRules] = React.useState([]);

	const incrementStage = React.useCallback(() => setStage(R.inc), [setStage]);

	return (
		<div>
			{
				[
					<ConfigureModel
						value={config}
						onChange={setConfig}
						incrementStage={incrementStage}
					/>,
					<GenerateRules
						config={config}
						rules={rules}
						onChange={setRules}
						incrementStage={incrementStage}
					/>,
					<RunCollapse config={config} rules={rules} />,
				][stage]
			}

			{true && <pre>{JSON.stringify(config, null, 2)}</pre>}
		</div>
	);
};

export default App;
