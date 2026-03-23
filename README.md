# Cleanup Taskbar

Our company rolls out some items to the taskbar that I do not need.

This is a little powershell script cleaning out those entries and reclaiming the
space on the taskbar. It runs periodically for 10min after every logon to catch
delayed rollout of the items. To avoid flashing console windows popping up,
there is a small wrapper-exe that calls the powershell script in the background.

## Build

Compile the project for the GNU windows target:

```
cargo build --release --target x86_64-pc-windows-gnu
```

## Install the Files

Copy the files to 'C:\usrbin\cleanup-taskbar'. If you change that path, you must
update the task accordingly.

## Install the Task

In an adminstrative shell run:

```
Register-ScheduledTask `
    -Xml (Get-Content 'C:\usrbin\cleanup-taskbar\Cleanup Taskbar.xml' | Out-String) `
    -User "YOUR_DOMAIN\YOUR_USER"
```