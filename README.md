# bintools
Replacement for unix commands on windows or macos. Written in pure rust, with standard library only as the goal. This project will be extended over time.

## Currently implemented:
| Command | Arguments |             Description            |
|---------|-----------|------------------------------------|
| touch   | File/Dir* | Create an empty file or directory. |
| rm      | File/Dir* | Delete an file or folder.          |

* Arguments marked with * (like File*) means that it accepts multiple arguments of that type.
* rm currently isn't a drop in replacement and mostly serves for basic deleting. It currently doesn't have an argument for recursive deletion, this will be added later, currently it always removes folders recursively.

## Next up:

| Command |           Arguments          |                           Description                           |
|---------|------------------------------|-----------------------------------------------------------------|
| ls      | #argument                    | List files and folders in the current directory.                |
| ls      | -l                           | List the files as actual list.                                  |
| ls      | -a                           | List hidden files and folders too.                              |
| ls      | -F                           | Add "/" at the end of folder names.                             |
| ls      | -t                           | Sort files and folders by modified time.                        |
| ll      |                              | Shorthand for ls -l                                             |
| cp      | #argument source destination | Copy a file or folder from one place to another.                |
| cp      | -f, --force                  | If the file already exists and can't be opened try to force it. |
| cp      | -i, --interactive            | Ask before overwriting existing files.                          |
| cp      | -n, --no-clobber             | Don't overwrite already existing files.                         |

## Overwriting PowerShell preset aliases
Find the corresponding profile.ps1 of your shell. If the actual file name shown on the commandline starts with 'Microsoft' just ignore it and go to the parent folder of the file.
```powershell
echo $profile
```
![example1](https://raw.githubusercontent.com/EKQRCalamity/bintools-rs/main/Screenshot%202024-02-26%20183834.png)

Edit the profile.ps1 with your favorite editor (Notepad is enough) and add the following line, replacing 'rm' with the preset you want to overwrite and 'rm.exe' with the thing you want it overwritten with. The stuff after that is for bypassing the restriction on overwriting presets.
```powershell
Set-Alias 'rm' 'rm.exe' -Force -Option 'Constant', 'AllScope'
```
![example2](https://raw.githubusercontent.com/EKQRCalamity/bintools-rs/main/Screenshot%202024-02-26%20183749.png)

You're done as long as you saved the file. From now on everytime you start that PowerShell instance the preset command will be overwritten.
