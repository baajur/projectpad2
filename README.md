# Projectpad

Projectpad allows to manage secret credentials and server information that you need to handle as a software developer. List of servers, list of point of interests on those servers (applications, log files, databases, servers). It will securely store passwords and keys. It will also allow you to run commands (locally or on SSH servers), open terminals on remote SSH servers, and so on.
The data is securely stored on-disk using [SQLcipher][], which uses 256-bit AES. The database is password-protected, but you can store the password in your OS keyring. Since the database is encrypted, you can put it in your dropbox (or similar account), to share it between computers.

Projectpad's target audience are people who today have to use several tools to do their work effectively: a keepass-like application, a series of text files with notes, and a few shell scripts. Instead of that, projectpad offers a streamlined, all-in-one solution.

Projectpad consists of two applications:

1. the GUI `projectpad` application, which allows you to enter/edit data, search it, open websites and so on;
2. the command-line `ppcli` application, which allows you to run commands, connect to servers, open files of interest and so on.

See [the help](https://github.com/emmanueltouzery/projectpad2/wiki/Help) for more details about the structure of the data that you can manage in projectpad.

## GUI application

The application allows you to manage your database of projects info. It is organized in three panes:

- projects
- project items (servers, project notes, project point of interests, server links)
- project item contents (for servers that may be a number of sub-items)

At the top of the second pane we can see the project environments (development, staging, uat and prod).

![Main view screenshot](https://raw.githubusercontent.com/wiki/emmanueltouzery/projectpad2/pics/gui1.png)

Notes are especially interesting, you author them in markdown syntax.

And full-text search is supported.

![search screenshot](https://raw.githubusercontent.com/wiki/emmanueltouzery/projectpad2/pics/gui2.png)

The application also supports operation in dark mode.

![dark theme screenshot](https://raw.githubusercontent.com/wiki/emmanueltouzery/projectpad2/pics/gui_dark1.png)

There was some effort made to make the GUI application as keyboard-friendly as possible.

## Command-line application

The command-line application loads all commands, servers, and files of interest, and displays them in a flat list, that you filter by typing and navigate using arrow keys. The application can execute commands, log you on ssh servers, edit configuration files, tail log files or fetch them, and so on.

![CLI1](https://raw.githubusercontent.com/wiki/emmanueltouzery/projectpad2/pics/cli1.svg)

In this second screenshot, the user typed 'sra fail' and therefore filtered the rows to display only only application servers (SRA) and matched 'fail' on the line, which matched the failover server.
Normally you would type keywords (part of the customer name, of the environment, of the item type), until the list is filtered to contain a few or a single element, at which you point you can just press enter to run the command.

![CLI2](https://raw.githubusercontent.com/wiki/emmanueltouzery/projectpad2/pics/cli2.svg)

[sqlcipher]: https://www.zetetic.net/sqlcipher/
