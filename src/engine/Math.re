let pi = [%bs.raw {| Math.PI |}];
let modulo: (int, int) => int = [%bs.raw {| (x, y) => x % y |}];
let sqrt: (float) => float = [%bs.raw {|  Math.sqrt |}];
let max: (float, float) => float = [%bs.raw {| Math.max |}];
let min: (float, float) => float = [%bs.raw {| Math.min |}];
let pow: (float, float) => float = [%bs.raw {| Math.pow |}];
let toFloat: (int) => float = float_of_int;
let toInt: (float) => int = int_of_float;
