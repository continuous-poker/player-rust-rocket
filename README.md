# Poker Player (Rust)

Your task is to implement the best poker player logic! :)

## Preparation
1. Choose a team name (PokerPenguins?)
2. Fork this repository
3. Add the following secret:
    * TOKEN (value will be provided by the facilitator)
4. Add the following variables:
    * NAMESPACE (value will be provided by the facilitator)
    * TEAMNAME (your team name - only lowercase letters, digits and dashes, must start with a lowercase letter)
4. Run the main workflow in GitHub - it will be deployed automatically
5. Notify the facilitator once you are done :)



## How it works

### Run it locally
```bash
cargo run
```
or
```bash
RUSTFLAGS="$RUSTFLAGS -A dead_code"
cargo build 
./target/debug/player-rust-rocket
```

### Your player bot
Your application provides a single endpoint at the root path (/).
This endpoint accepts a table object, which gives you an overview about the
current state of the game.
You will be able to see the other players and their bets. You can also see
the community cards as well as your own cards. Using this information and given
your current stack of chips, you have to decide how much you want to bet.

### Poker game
If all players have successfully deployed their application and registered it
with the game, the dealer service will start playing tournaments over and over.
Each player has a starting stack of 100 chips. The blinds will start low but increase over time in each tournament.
The player who wins the tournament receives 1 point on the total scoreboard.
After that, the next tournament starts automatically.

### Betting round
Each time a betting round starts, the dealer application will call the player bots one by one and
retrieve their bets. This is a sample JSON of the table information provided to you:
```json
{
  "activePlayer": 2,
  "communityCards": [
    {
      "rank": "10",
      "suit": "HEARTS"
    },
    {
      "rank": "5",
      "suit": "CLUBS"
    },
    {
      "rank": "A",
      "suit": "DIAMONDS"
    }
  ],
  "currentDealer" : 0,
  "minimumBet": 20,
  "minimumRaise": 40,
  "players": [
    {
      "name": "Bot1",
      "status": "ACTIVE",
      "stack": 990,
      "bet": 10
    },
    {
      "name": "Bot2",
      "status": "ACTIVE",
      "stack": 980,
      "bet": 20
    },
    {
      "name": "Bot3",
      "status": "ACTIVE",
      "stack": 1000,
      "bet": 0,
      "cards": [
        {
          "rank": "2",
          "suit": "HEARTS"
        },
        {
          "rank": "J",
          "suit": "CLUBS"
        }
      ]
    }
  ],
  "pot": 0,
  "round": 1,
  "smallBlind": 10
}
```
## Start the game
Now it's your turn! Start implementing the Strategy function to do more than just fold every turn.
Or rework the whole application. You can do what you want, as long as you provide the endpoint that is registered in the game.
Have fun!

## Contribution

Please refer to our [contribution guidelines](CONTRIBUTING.md) if you wish to contribute to our open source project.

## License

This program and the accompanying materials are made available under the terms
of the Apache License, Version 2.0 which is available at
https://www.apache.org/licenses/LICENSE-2.0.

SPDX-License-Identifier: Apache-2.0