
# Snake Game in Rust

Welcome to the Snake Game project in Rust! This is a simple text-based snake game that runs in the terminal. The game is built using the [crossterm](https://crates.io/crates/crossterm) crate to handle terminal rendering and user input.

## How to Play

- Use the arrow keys to move the snake:
  - **Up**: Move the snake upwards.
  - **Down**: Move the snake downwards.
  - **Left**: Move the snake leftwards.
  - **Right**: Move the snake rightwards.
- Press **q** to quit the game.

The snake moves continuously, and the player must direct its movement using the arrow keys.

## Project Structure

- `Cargo.toml`: Project metadata and dependencies.
- `src/main.rs`: Main logic for the Snake Game, including terminal control and snake movement.

## Setup Instructions

1. **Clone the repository**:

   ```bash
   git clone https://github.com/the-real-jerry-tan/SnakeGameX.git
   cd SnakeGameX
   ```

2. **Build the project**:

   Ensure that you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

   ```bash
   cargo build
   ```

3. **Run the game**:

   After building the project, run the game using:

   ```bash
   cargo run
   ```

4. **Play the game**:

   Use the arrow keys to control the snake and press **q** to quit.

## Next Steps for Enhancement

Here are some ideas to further enhance the Snake Game:

1. **Food and Scoring**: Implement food that the snake can eat, and keep track of the score. The snake should grow longer when it eats food.
2. **Collision Detection**: Implement game-over conditions when the snake runs into the walls or itself.
3. **Levels**: Add different levels with increased difficulty (e.g., increased speed, obstacles).
4. **Improve TUI**: Enhance the terminal user interface with colors, borders, or a better layout.
5. **Pause Feature**: Add the ability to pause and resume the game.
6. **Leaderboard**: Save and display the highest scores across multiple game sessions.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributions

Feel free to fork the repository and submit pull requests with improvements or bug fixes.

---

Happy Coding! 🐍
