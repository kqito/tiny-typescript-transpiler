const obj = {
  a: 1,
  b: "2",
  c: true,
  d: null,
};

function binary(a: number, b: number): number {
  return a + b;
}

if (obj.a) {
  console.log(binary(obj.a, 2));
}
