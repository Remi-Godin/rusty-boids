### Rusty Boids
This is my second project made in Rust: a naive implementation of boids.

https://github.com/Remi-Godin/rusty-boids/assets/129818497/ee69665a-cdd7-4306-8c17-a1d36bbf87c7

#### Project Description
I said naive because I am simply making it up as I go. The goal I set myself was to implement boids in Rust knowing only the three rules of boids, without knowing their proper implementation. The final result is not quite perfect, but I think it qualifies as a boid implementation.

The structure I decided to follow was to have a struct for each boid and a struct for the flock (all the boids). Inside the flock struct, I hold the force vectors for each boids, add the different forces together to get a final force vector, which is then applied to the boid. The magnitude of the final force vector does not matter since the boids have a fixed speed, but it is nesseray for calculating the final vector from the 3 rules.


