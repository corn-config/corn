import * as corn from 'corn/index.mjs';

let result = corn.parse('{foo = "bar"}').value;
console.log(result);