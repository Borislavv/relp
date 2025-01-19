# REPL - remote executor helper.

#### For macOS:
If you want to handle the REPL is running always, you probably would like set uo the REPL as launchctl service.
How to set it up? At first, you need copy the example .plist file in the root of project into the target directory: /Library/LaunchDaemons.
Next step is modifying the target .plist and set up the correct environment variables.
That's all, now, you can run a "make" command in the project root dir.