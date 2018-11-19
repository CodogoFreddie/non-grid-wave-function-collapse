type state = int;
type stateProbability = float;
type stateSuperposition = array(stateProbability);
type cell = stateSuperposition;
type medium = {
  width: int,
  height: int,
  numberOfStates: int,
  cells: array(array(cell)),
};

type rule = {
  center: state,
  outers: array(state),
};

let createMedium = (~width, ~height, ~numberOfStates) => {
  width,
  height,
  numberOfStates,
  cells: Array.make_matrix(width, height, Array.make(numberOfStates, 1.0)),
};

let pi = [%bs.raw {| Math.PI |}];
let modulo: (int, int) => int = [%bs.raw {| (x, y) => x % y |}];

let samplePointFromMedium =
    (cx: int, cy: int, r: int, m: medium, numberOfStates: int)
    : stateSuperposition => {
  let area = pi *. float_of_int(r) *. float_of_int(r);
  let acc: stateSuperposition = Array.make(m.numberOfStates, 0.0);

  for (i in - r to r) {
    let x = modulo(i, m.width);
    for (j in - r to r) {
      let y = modulo(j, m.width);
      for (s in 0 to numberOfStates) {
        acc[s] = acc[s] +. m.cells[x][y][s];
      };
    };
  };

  for (s in 0 to numberOfStates) {
    acc[s] = acc[s] /. area;
  };

  acc;
};

type runTickType = unit => medium;
let createEngine =
    (~width: int, ~height: int, ~numberOfStates: int, numberOfSamples: int)
    : runTickType => {
  let medium = createMedium(~width, ~height, ~numberOfStates);

  () => medium;
};
