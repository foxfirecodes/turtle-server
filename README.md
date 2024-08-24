# turtle-server

> tiny server config for a lil server i call "turtle" (just for a few friends)

## setup

during first time setup you will want to run `get-env.sh` to initialize `.env` with the appropriate `UID` and `GID` values.

from there, it's just a matter of configuring some optional env vars if you want the Discord bot to work, namely:

### `.env`

- `CFG_DISCORD_TOKEN` - the bot token for your Discord bot (for MC <-> Discord integration)
- `CFG_DISCORD_CHANNEL` - the channel ID where you want the bot to send server & player messages
- `CFG_DISCORD_ADMIN_ROLE` - the role ID for gating access to admin-only commands (like `/stop` from within Discord)

### `bot/.env`

- `DISCORD_TOKEN` - probably same as `CFG_DISCORD_TOKEN` in `.env`. sets up a bot w/ a slash command to start the server if it ever dies for whatever reason

## usage

just `docker compose up -d` from within the `turtle-server` directory to start, `docker compose down` to stop

you can `docker compose attach mc` to access the interactive console. use <kbd>Ctrl-p Ctrl-q</kbd> to detach
