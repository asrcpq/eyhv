Still in its early development stage

## Key features

* High speed and high fault tolerance

* Minimalism design

* Randomly generated and reproducible(by u64 seed) enemies

* Time in game always synchronized: player will suffer instead of benefit from low FPS

* Everything is drawed, no resource file, GUI frontend is replaceable

## TODO

### Implementation

* [x] Moving

* [x] Restrict position

* [x] Slowdown

* [x] Color and transparency

* [x] Antialiasing

* [x] World slowdown with transition

* [x] Simple cannon & simple bullet

* [x] Enemy path and graphics

* [x] Simple collision test with position(not tested!)

* [x] Collision box for enemy and player bullet(lineseg\_distance function)

* [x] Enemy cannon

* [x] Reproducible randomize cannon and bullet param(Framework)

* [x] Wave generator and wave manager
(add 1 wave type contains of 1 type of enemy with 1 type of cannon)

* [x] Player collision box

* [ ] Add more contents

	* [x] New Cannon: Rotor with new bullet: RotateBullet

	* [x] New GraphicObject and modify GraphicObjects

	* [ ] New WaveScheme, Enemy, EnemyPath

* [ ] Enemy HP

* [x] simple\_try random generator optimization with monotonicity

* [ ] Game mechanism

* [ ] Visual effects

### Coding

* [ ] Generalize tick() by using timer interface

* [ ] Fine-grained inter-module visibility

* [ ] Using reference instead of cloning everything everywhere(necessary?)

### Performance

* [ ] Asynchronous wave generator

* [ ] Parallelize
