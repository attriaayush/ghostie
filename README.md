# ghostie

> Github notifications in your terminal. Available on MacOS and Linux.

![](./docs/demo.gif)

## Features

- Runs as background process, fetching new github notifications in a 48h rolling
    window.
- Polls every 1 minute and uses SQLite to persist local cache of notifications.
- Issues desktop notification (MacOS only for now) when new notifications
    are received.
- View notifications in your terminal and opens them in your default browser when
    selected.

## Installation

## Usage

Simply run `ghostie` and see the list of things that Ghostie supports.

**Note: To view the notifications using `ghostie view` ensure `ghostie` is running
as a background process.**

```
ghostie
manage your github notifications in terminal

USAGE:
    ghostie <SUBCOMMAND>

SUBCOMMANDS:
    clear-logs    Clear logs from the background process
    count -C      Query the count of unread github notifications
    logs -L       Show logs from the background process
    prune -P      Prune all notifications from the local cache
    start         Run ghostie as a background process
    stop          Stop ghostie as a background process
    view -V       Open UI to manage github notifications
```

