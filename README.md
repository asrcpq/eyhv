Still in its early development stage

## Key features

* High speed and high fault tolerance(of player performance) Minimalist design(game mechanism and design style) 
* Randomly generated and reproducible(by u64 seed) enemies

* Time in game always synchronized: player will suffer, not benefit from low FPS

* Everything drawed, no resource file, replaceable GUI frontend

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

* [x] simple\_try random generator optimization with monotonicity

* [x] Enemy HP

* Add more resources

	* [x] New Cannon: Rotor with new bullet: RotateBullet

	* [x] New EnemyPrototype: Medium

	* [x] New Cannon: Shotgun

	* [x] New Cannon: LaserLocker

	* [ ] New Cannon: LaserSlicer

	* [ ] New Cannon: Sweeper

* [x] Pause

* [x] WavePool

* Fix

	* [x] Fix lag decrease hit possibility
	
	* [ ] Fix overlapping of waves
	
	* [ ] Enemy should not always fire

* [x] Player moves faster when not firing

* [x] Player hit reset

* [ ] PlayerCannon2

* [ ] Game mechanism

* [ ] Visual effects

	* [ ] Draw collision box in slowdown mode

	* [ ] Field background

	* [ ] Destroy effect

### Coding

* [x] Split cannons file

### Performance

* [ ] SDL2 has poor draw performance, switch to raw OpenGL backend interface

* [ ] Asynchronous wave generator

* [ ] Parallelize
