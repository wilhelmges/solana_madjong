# Generates the 36 placeholder tile images (256x256 PNG) into assets\tiles\
# Run: powershell -ExecutionPolicy Bypass -File tools\gen_tile_images.ps1
# Replace individual files later with high-quality art (same file names).

Add-Type -AssemblyName System.Drawing

$out = Join-Path $PSScriptRoot "..\assets\tiles"
New-Item -ItemType Directory -Force -Path $out | Out-Null

# (name, r, g, b) — keep in sync with src/theme/solana.rs
$tiles = @(
    @("SOL", 150, 50, 200), @("USDC", 0, 200, 100), @("USDT", 200, 100, 0),
    @("BONK", 200, 50, 50), @("WIF", 50, 200, 200), @("JUP", 100, 200, 100),
    @("RAY", 200, 200, 50), @("ORCA", 50, 100, 200), @("PYTH", 200, 150, 100),
    @("JTO", 150, 200, 50), @("Swap", 0, 150, 200), @("Stake", 0, 200, 150),
    @("Lend", 50, 200, 200), @("Yield", 100, 200, 150), @("Bridge", 0, 180, 180),
    @("DAO", 50, 150, 200), @("NFT", 100, 180, 200), @("Mint", 0, 200, 180),
    @("Pool", 50, 200, 180), @("Farm", 100, 200, 180), @("Saga", 180, 0, 180),
    @("Firedancer", 200, 0, 150), @("Token2022", 220, 50, 200), @("Program", 200, 0, 200),
    @("Account", 180, 50, 180), @("Block", 200, 100, 200), @("Vote", 180, 0, 200),
    @("Wallet", 200, 50, 180), @("Jito", 50, 50, 200), @("Marinade", 100, 50, 200),
    @("Lido", 150, 50, 200), @("Rocket", 200, 50, 200), @("Figment", 50, 100, 200),
    @("Anza", 100, 100, 200), @("Triton", 150, 100, 200), @("Helius", 200, 100, 200)
)

foreach ($t in $tiles) {
    $name = $t[0]
    $bmp = New-Object System.Drawing.Bitmap 256, 256
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    # base colored face
    $g.Clear([System.Drawing.Color]::FromArgb(255, [int]$t[1], [int]$t[2], [int]$t[3]))

    # white frame + darker bottom edge for 3D feel
    $framePen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(255, 255, 255, 255), 7)
    $g.DrawRectangle($framePen, 5, 5, 246, 246)
    $darkPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(90, 20, 20, 28), 10)
    $g.DrawLine($darkPen, 12, 244, 244, 244)

    # centered name
    $len = $name.Length
    $fs = [Math]::Max(24, [Math]::Min(84, [int](400 / $len)))
    $font = New-Object System.Drawing.Font("Arial", $fs, [System.Drawing.FontStyle]::Bold, [System.Drawing.GraphicsUnit]::Pixel)
    $brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
    $format = New-Object System.Drawing.StringFormat
    $format.Alignment = [System.Drawing.StringAlignment]::Center
    $format.LineAlignment = [System.Drawing.StringAlignment]::Center
    $rect = New-Object System.Drawing.RectangleF(0, 0, 256, 256)
    $g.DrawString($name, $font, $brush, $rect, $format)

    $path = Join-Path $out ($name.ToLowerInvariant() + ".png")
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)

    $g.Dispose(); $bmp.Dispose(); $font.Dispose(); $brush.Dispose(); $format.Dispose()
    $framePen.Dispose(); $darkPen.Dispose()

    Write-Host "wrote $path"
}
Write-Host "Done: $($tiles.Count) images"