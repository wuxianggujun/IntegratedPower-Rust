Param(
    [ValidateSet('debug','release')]
    [string]$Configuration = 'release'
)

# 复制 resources 目录到构建输出目录（优先 bin，其次 target）
$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path | Split-Path -Parent
$resourcesDir = Join-Path $repoRoot 'resources'

$candidates = @('bin','target') | ForEach-Object { Join-Path $repoRoot $_ }
$targetRoot = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $targetRoot) { $targetRoot = Join-Path $repoRoot 'bin' }

$outDir = Join-Path $targetRoot $Configuration

if (-not (Test-Path $resourcesDir)) {
    Write-Host "[copy_resources] 未找到资源目录: $resourcesDir" -ForegroundColor Yellow
    exit 0
}

if (-not (Test-Path $outDir)) {
    Write-Host "[copy_resources] 输出目录不存在: $outDir，请先执行 cargo build --$Configuration" -ForegroundColor Yellow
    exit 0
}

$dest = Join-Path $outDir 'resources'
if (Test-Path $dest) {
    Write-Host "[copy_resources] 清理旧的目标资源: $dest"
    Remove-Item -Recurse -Force $dest
}

Write-Host "[copy_resources] 复制 $resourcesDir -> $dest"
Copy-Item -Recurse -Force $resourcesDir $dest

Write-Host "[copy_resources] 完成"
