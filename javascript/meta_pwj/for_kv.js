function logDairy() {
  var dairy = [
    "cheese",
    "sour cream",
    "milk",
    "yogurt",
    "ice cream",
    "milkshake",
  ];

  for (const item of dairy) {
    console.log(item);
  }
}

function birdCan() {
  const animal = {
    canJump: true,
  };

  const bird = Object.create(animal);
  bird.canFly = true;
  bird.hasFeathers = true;

  for (const key of Object.keys(bird)) {
    console.log(`${key}: ${bird[key]}`);
  }

  return bird;
}

function animalCan(bird) {
  for (const key in bird) {
    console.log(`${key}: ${bird[key]}`);
  }
}

logDairy();
const bird = birdCan();
animalCan(bird);
