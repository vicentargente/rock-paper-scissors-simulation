# Rock, Paper, Scissors - A Bevy ECS Experiment

This is a simple simulation built with the [Bevy game engine](https://bevyengine.org/) to learn about its Entity Component System (ECS). It's just for fun and to see how many entities can be handled smoothly.

## The Simulation Rules

The simulation follows the classic rules of Rock, Paper, Scissors:

  * **Rock** beats **Scissors**
  * **Scissors** beats **Paper**
  * **Paper** beats **Rock**

When two entities from different teams touch, one will convert the other to its team based on these rules. The simulation ends when only one team remains.

<img src="https://i.imgur.com/sMiFOSc.jpeg"></img>

## How to Run

To run the simulation, you'll need to have Rust and Cargo installed. Then, simply clone the repository and run the following command in the project directory:

```bash
cargo run --release
```

Enjoy the chaos\!