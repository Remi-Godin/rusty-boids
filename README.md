### Rusty Boids
This is my second project made in Rust.

https://github.com/Remi-Godin/rusty-boids/assets/129818497/ee69665a-cdd7-4306-8c17-a1d36bbf87c7

#### Project Description
The goal I set myself was to implement boids in Rust knowing only the three rules of boids, without knowing their proper implementation or optimization strategy. It was more of a project to get my feet wet with rust. When I made this project, my knowledge of data structures and algorithms wasn't as good as it is now, so was my knowledge of rust as a language, but I learned a lot with this project and later on I'd like to try it again from scratch and see how performant I can make it.

The structure I decided to follow was to have a struct for each boid and a struct for the flock (all the boids). Inside the flock struct, I hold the force vectors for each boids, add the different forces together to get a final force vector, which is then applied to the boid.
