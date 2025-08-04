let set = new Set();
set.add(1);
set.add(2);
set.add(3);
set.add(2);
set.add(1);

console.log(set); // Set { 1, 2, 3 }

let food = "Chocolate";
console.log(`My favourite food is ${food}`);

const meal = ["soup", "steak", "ice cream"];
let [starter] = meal;
console.log(starter);
