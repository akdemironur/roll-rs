# roll-rs

roll-rs is a Discord bot for rolling dice, designed for tabletop RPG enthusiasts and anyone who needs a dice roller. Built with Rust, it offers fast and efficient dice rolling with support for standard dice notation and advanced "keep highest" rolls.

## Features

- **Standard Dice Rolling**: Supports the classic `XdY±Z` notation (e.g., `3d6+2`).
- **"Keep Highest" Rolls**: Allows rolling multiple dice and keeping only the highest results (e.g., `keep 3 5d6+5`).
- **Saved Rolls**: Users can save custom roll configurations for quick access.
- **Discord Slash Commands**: Easy-to-use slash command interface.

## Requirements

- Rust (latest stable version)
- Discord Bot Token

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/roll-rs.git
   cd roll-rs
   ```

2. Set up your Discord bot and get your token from the [Discord Developer Portal](https://discord.com/developers/applications).

3. Set the Discord token as an environment variable:
   ```
   export DISCORD_TOKEN=your_token_here
   ```

4. Build and run the bot:
   ```
   cargo run
   ```

## Usage

roll-rs uses Discord's slash commands for easy interaction. Here are some examples:

- `/r 3d6+2`: Roll three 6-sided dice and add 2 to the result.
- `/r keep 3 5d6+5`: Roll five 6-sided dice, keep the highest 3, and add 5 to the sum.
- `/save 3d6+2 attack`: Save a roll configuration named "attack".
- `/r attack`: Use a saved roll configuration.
- `/view`: View saved rolls.
- `/remove attack`: Remove `attack` from saved rolls.

## Commands

- `/r [dice notation]`: Roll dice using standard notation or "keep highest" notation.
- `/save [dice notation] [name]`: Save a dice roll configuration.
- `/hello`: A simple greeting command to check if the bot is responsive.

## Advanced Usage

### Standard Notation
The standard dice notation is in the format `XdY±Z`, where:
- `X` is the number of dice to roll
- `Y` is the number of sides on each die
- `±Z` is an optional modifier to add or subtract from the total

Example: `/r 3d6+2`

### "Keep Highest" Notation
The "keep highest" notation is in the format `keep N XdY±Z`, where:
- `N` is the number of highest dice to keep
- `X` is the number of dice to roll
- `Y` is the number of sides on each die
- `±Z` is an optional modifier to add or subtract from the total

Example: `/r keep 3 5d6+5`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [serenity](https://github.com/serenity-rs/serenity) and [poise](https://github.com/serenity-rs/poise) for Discord bot functionality.
- Uses [rand](https://github.com/rust-random/rand) for random number generation.