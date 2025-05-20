# 获取当前脚本所在目录，作为 cargo 项目的根目录
$projectDir = Get-Location

# 定义服务器列表
$servers = @(
    "wicked-waifus-config-server",
    "wicked-waifus-hotpatch-server",
    "wicked-waifus-login-server",
    "wicked-waifus-gateway-server",
    "wicked-waifus-game-server"
)

# 构建 wt 命令行参数
$wtArgs = ""

foreach ($i in 0..($servers.Count - 1)) {
    $server = $servers[$i]
    $title = $server
    $command = "powershell -NoExit -Command `"cargo run --bin $server`""
    $startDir = $projectDir.Path

    if ($i -eq 0) {
        $wtArgs += "new-tab --title `"$title`" --startingDirectory `"$startDir`" $command"
    } else {
        $wtArgs += " ; new-tab --title `"$title`" --startingDirectory `"$startDir`" $command"
    }
}

# 启动 wt 并运行所有标签页
Start-Process wt -ArgumentList $wtArgs
