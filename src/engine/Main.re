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

let samplePointFromMedium = (cx: int, cy: int, r: int, m: medium, numberOfStates: int): stateSuperposition => {
  let area = Math.pi *. Math.toFloat(r) *. Math.toFloat(r);
  let acc: stateSuperposition = Array.make(m.numberOfStates, 0.0);

  for (i in - r to r) {
    let x = Math.modulo(i, m.width) + cx;
    for (j in - r to r) {
      let y = Math.modulo(j, m.width) + cy;
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

let paintPosibilityEliminationOnMedium = (cx: int, cy: int, r: int, state: state, m: medium): medium => {
  let {width, height} = m;

  for (i in - r to r) {
    let x = Math.modulo(i, m.width);
    for (j in - r to r) {
      let y = Math.modulo(j, m.height);
      let dx: float =
        Math.min(Math.min(Math.toFloat(x - cx), Math.toFloat(x - width - cx)), Math.toFloat(x + width - cx));
      let dy: float =
        Math.min(Math.min(Math.toFloat(y - cy), Math.toFloat(y - height - cy)), Math.toFloat(y + height - cy));
      let d: float = Math.sqrt(dx *. dx +. dy *. dy);

      m.cells[x][y][state] = Math.max(m.cells[x][y][state] -. d /. Math.toFloat(r), 0.0);
    };
  };

  m
};

type runTickType = unit => medium;
let createEngine = (~width: int, ~height: int, ~numberOfStates: int ): runTickType => {
  let medium = createMedium(~width, ~height, ~numberOfStates);

  () => medium;
};
