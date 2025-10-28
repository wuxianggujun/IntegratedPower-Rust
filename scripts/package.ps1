Param(
    [ValidateSet('debug','release')]
    [string]$Configuration = 'release',
    [string]$OutDir = 'dist'
)

$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path | Split-Path -Parent
$targetCandidates = @('bin','target') | ForEach-Object { Join-Path $repoRoot $_ }
$targetDir = $targetCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $targetDir) { $targetDir = Join-Path $repoRoot 'bin' }

# 1) 构建
Write-Host "[package] 构建 $Configuration..." -ForegroundColor Cyan
if ($Configuration -eq 'release') {
    cargo build --release
} else {
    cargo build
}

# 2) 路径与文件名
$exeName = 'integrated-power.exe'
$exeSource = Join-Path (Join-Path $targetDir $Configuration) $exeName
if (-not (Test-Path $exeSource)) {
    throw "未找到可执行文件: $exeSource"
}

$outBase = Join-Path $repoRoot $OutDir
$bundleDir = Join-Path $outBase $Configuration

if (Test-Path $bundleDir) {
    Write-Host "[package] 清理旧目录: $bundleDir"
    Remove-Item -Recurse -Force $bundleDir
}
New-Item -ItemType Directory -Force -Path $bundleDir | Out-Null

# 3) 复制可执行文件
Copy-Item -Force $exeSource (Join-Path $bundleDir $exeName)

# 4) 复制资源目录（优先使用 target/<cfg>/resources，其次 repoRoot/resources）
$resFromTarget = Join-Path (Join-Path $targetDir $Configuration) 'resources'
$resFromRepo = Join-Path $repoRoot 'resources'

if (Test-Path $resFromTarget) {
    Write-Host "[package] 复制资源: $resFromTarget -> $(Join-Path $bundleDir 'resources')"
    Copy-Item -Recurse -Force $resFromTarget (Join-Path $bundleDir 'resources')
} elseif (Test-Path $resFromRepo) {
    Write-Host "[package] 复制资源: $resFromRepo -> $(Join-Path $bundleDir 'resources')"
    Copy-Item -Recurse -Force $resFromRepo (Join-Path $bundleDir 'resources')
} else {
    Write-Host "[package] 未找到资源目录，跳过复制。" -ForegroundColor Yellow
}

Write-Host "[package] 完成，输出目录: $bundleDir" -ForegroundColor Green
