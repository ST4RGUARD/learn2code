class Person {
    constructor(name = "Tom", age = 20, energy = 100) {
        this.name = name;
        this.age = age;
        this.energy = energy
    }


    sleep() {
        this.energy += 10;
    }

    doSomethingFun() {
        this.energy -= 10;
    }

}

class Worker extends Person {
  constructor(name, age, energy, xp = 0, hourlyWage = 10) {
    super(name, age, energy);
    this.xp = xp;
    this.hourlyWage = hourlyWage;
  }

  goToWork() {
    this.xp += 10;
  }
}

function intern() {
  const internObj = new Worker("Bob", 21, 110, 0, 10);
  internObj.goToWork();
  return internObj;
}

const bob = intern();
console.log(bob);

function manager() {
    const managerObj = new Worker("Alice", 30, 120, 100, 30);
    managerObj.doSomethingFun();
    return managerObj;
}
