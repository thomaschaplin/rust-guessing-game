# rust-guessing-game

Number guessing game written in [rust](https://www.rust-lang.org/)

## Setup

Make sure you have [rust](https://www.rust-lang.org/) installed on your machine by following the [getting started guide](https://www.rust-lang.org/learn/get-started)

## Instructions

* Clone this repository `git clone git@github.com:thomaschaplin/rust-guessing-game.git`
* Change directory `cd rust-guessing-game`
* Build the application `cargo build`
* Run the application `cargo run`

#### Example output:

```
Guess the number between 1 & 100

Please input your guess.

28
28 is too small, try again!
99
99 is too large, try again!
54
54 is correct, you win!
```

# Docker Setup

Build
```
docker build --rm -f Dockerfile -t thomaschaplin:rust-guessing-game .
```

Run
```
docker run --rm -it thomaschaplin:rust-guessing-game
```
