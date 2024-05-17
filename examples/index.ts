type Str = String;
const str: Str = "hogehoge";

const a: number = 100;
const b: string = "hello";
const c: RegExp = /test/;
const d: boolean = true;
const e: boolean = false;
const f = undefined;
const g = null;
const h = 0b101;

var numTest1 = 100;
let numTest2 = 100;
const numTest3 = numTest2;

function alertMessage(message: string) {
  alert(message);
}

alertMessage("Hello, TypeScript!");
