# Guessing Game

This is a simple guessing game program written in Rust. The goal of the game is to guess a randomly generated secret number within a certain range (1 to 10) in a limited number of attempts.

## Prerequisites

To run this program, make sure you have Rust installed on your system. You can install Rust from the official [website](https://www.rust-lang.org/tools/install)



## Dependencies
The following dependencies are used in this program:

1) `rand`: A Rust library for generating random numbers. This is used to generate the secret number that the player has to guess.

2) `std::cmp`: A Rust module that provides the Ordering enumeration for comparing values. This is used to compare the guessed number with the secret number.

3) `std::io`: A Rust module that provides standard input and output facilities. This is used for reading the user's input and printing messages to the console.


## Usage
Clone the repository:

```bash
git clone https://github.com/surajsah2053/guessing_game.git
cd guessing_game
```

## Run the program:

```bash
cargo run
```

This will compile and run the program. Follow the on-screen instructions to play the game. Good luck!


## How to play

```
1) The program generates a random secret number between 1 and 10.
2) The player has 3 attempts to guess the secret number.
3) In each attempt, the player inputs their guess, and the program provides feedback on whether the guess is too high, too low, or correct.
4) If the player guesses the secret number correctly, they win, and the game ends. The game is over if they fail to guess the correct number within 3 attempts.

```

## Contributing
```
If you'd like to contribute to this project, feel free to fork the repository, 
create a branch with your changes, and submit a pull request. All contributions are welcome!