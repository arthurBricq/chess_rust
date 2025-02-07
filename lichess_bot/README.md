# Deploy chess-engine on Lichess

The following command deploys the chess engine on Lichess, on my personal `yoda-bot` lichess-account.

```bash
cd /Users/arthurbricq/dev/python/lichess-bot && python3 lichess-bot.py --config /Users/arthurbricq/dev/rust/chess_rust/lichess_bot/config.yaml
```

## Current state

- The engine is not good at end-games, or at checkmating.
