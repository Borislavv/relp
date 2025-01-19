# REPL - remote executor helper.

#### For macOS:
If you want to ensure that the REPL is always running, you would probably like to set up the REPL as a launchctl service. How to set it up? First, you need to copy the example .plist file from the root of the project to the target directory: /Library/LaunchDaemons. The next step is to modify the target .plist and set up the correct environment variables. That's all. Now you can run the make command in the project root directory.

#### For linux:
This repository does not have installation instructions for Linux, but you can easily find documentation for setting up a new service with systemctl for use by systemd.
