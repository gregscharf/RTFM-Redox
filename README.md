
The goal of this project is to create an easier to use and updated replacement for [RTFM](https://github.com/leostat/rtfm) or something to run in your terminal that works like [Rev Shells](https://revshells.com/) or the [HackTools](https://addons.mozilla.org/en-US/firefox/addon/hacktools/) browser plugin. This started as a project to help improve my proficiency with Rust. I'm still new to it so I'm sure there are some poor Rust coding practices but I will be refactoring the code as I go.  I'm currently using the sqlite database that [RTFM](https://github.com/leostat/rtfm) uses but that needs some updating and new syntax needs to be added since that doesn't seem to have been updated in almost 6 years.  

My current usage of this is to have it running in a tmux pane and when I need the syntax for something like downloading a file from my local machine and then executing that in memory via powershell then I can just ctrl+r in the CLI, type something like IEX or powershell, and then select the syntax I need via the arrow keys.  Pressing return on the hightlighted command will then copy that command to the clipboard.  So instead of searching through my notes or opening a web browser, I can stay in the terminal and quickly copy/paste the command I need via the Red(ox) CLI.  

5-25-23: CLI variable replacement for commands will be added within the week. 

05-25-23: This is still very rough but it is at a point where it is a functional proof of concept.  Currently you can add commands to the database from the CLI, search with the 'search' command or quick search with ctrl+r similar to searching through your terminal history.  Once results are returned you can use the up/down arrow keys to highlight a command and then press return to copy that command to your clipboard.  

## To Add
- [ ] Set variables for replacement of things like Remote Host, Local Host, Local Port, etc similar to msfconsole.
- [x] Display command's comment and any other info after it has been selected.
- [ ] Add related commands associated with a command
    - In the full search output these would also be listed out under related commands
- [ ] Implement [RTFM](https://github.com/leostat/rtfm)'s solution for creating/updating the database
- [ ] Update the database with newer commands for things like Bloodhound,Rubeus,Crackmapexec,Chisel,SSHuttle, various potato attacks, etc.
- [ ] Add search capability for text based/markdown notes.  
    - Root directory for notes will be supplied in a CLI variable.

## To Fix
- [ ] Add buffer to scroll through search output that doesn't fit within terminal windows.
    - Current behavior is unhandled and will crash the application
- [ ] Termion screen refresh on MAC ARM causes screen to flicker when arrowing through commands.
