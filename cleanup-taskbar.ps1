function unpin_taskbar([string]$appname) {
    $NAMESPACE='shell:::{4234d49b-0245-4df3-b780-3893943456e1}'
    $UNPIN='Unpin from taskbar'
	(
		(New-Object -Com Shell.Application).NameSpace($NAMESPACE).Items() |
		? {$_.Name -eq $appname}
	) |
	% {
		$_.Verbs() |
		? {$_.Name.replace('&','') -eq $UNPIN} |
		% { $_.DoIt() }
	}
}

$DIR_USER=$env:USERPROFILE
$DIR_ROAMING="$DIR_USER\AppData\Roaming"
$DIR_MS="$DIR_ROAMING\Microsoft"
$DIR_STARTMENU="$DIR_MS\Windows\Start Menu\Programs"
$DIR_TASKBAR="$DIR_MS\Internet Explorer\Quick Launch\User Pinned\TaskBar"

rm -ErrorAction SilentlyContinue "$DIR_STARTMENU\File Explorer.lnk"
rm -ErrorAction SilentlyContinue "$DIR_TASKBAR\File Explorer.lnk"
rm -ErrorAction SilentlyContinue "$DIR_TASKBAR\Microsoft Teams classic (work or school).lnk"

unpin_taskbar("Company Portal")
unpin_taskbar("File Explorer")
unpin_taskbar("Microsoft Teams")
