[CmdletBinding()]
param(
  [string]$Destination = $(Join-Path $HOME ".local\bin"),
  [string]$Tag,
  [string]$Token
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Ensure-WindowsPlatform {
  if (-not [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Windows)) {
    Write-Error "This installer targets Windows. Use scripts/install.sh on Unix-like systems."
    exit 1
  }
}

function Ensure-SupportedArchitecture {
  $architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
  if ($architecture -ne [System.Runtime.InteropServices.Architecture]::X64) {
    Write-Error "Unsupported architecture '$architecture'. Only x64 Windows binaries are published."
    exit 1
  }
}

function Resolve-GitHubToken {
  param([string]$ExplicitToken)

  foreach ($candidate in @($ExplicitToken, $env:GITHUB_TOKEN, $env:GH_TOKEN)) {
    if ($candidate) {
      return $candidate
    }
  }

  return $null
}

function Get-GitHubHeaders {
  param([string]$Token)

  $headers = @{
    "Accept" = "application/vnd.github+json"
    "X-GitHub-Api-Version" = "2022-11-28"
  }

  if ($Token) {
    $headers["Authorization"] = "Bearer $Token"
  }

  return $headers
}

function Get-GitHubRelease {
  param(
    [string]$Url,
    [hashtable]$Headers
  )

  try {
    return Invoke-RestMethod -Uri $Url -Headers $Headers -ErrorAction Stop
  } catch {
    throw "Failed to query GitHub releases: $($_.Exception.Message)"
  }
}

function Select-ReleaseAsset {
  param(
    $Release,
    [string]$Target
  )

  $asset = $Release.assets | Where-Object { $_.name -like "*$Target.zip" } | Select-Object -First 1
  if ($asset) {
    return $asset
  }

  $name = if ($Release.tag_name) { $Release.tag_name } elseif ($Release.name) { $Release.name } else { "the selected release" }
  throw "No release asset matching $Target was found in $name."
}

function New-TemporaryDirectory {
  $path = Join-Path ([System.IO.Path]::GetTempPath()) ("shelltape-" + [Guid]::NewGuid().ToString("N"))
  New-Item -ItemType Directory -Path $path | Out-Null
  return $path
}

function Ensure-PathContainsDestination {
  param([string]$Destination)

  $pathEntries = ($env:PATH -split ';') | Where-Object { $_ }
  if ($pathEntries -notcontains $Destination) {
    Write-Warning "Add '$Destination' to your PATH to use shelltape from any shell."
  }
}

function Install-ShelltapeBinary {
  param(
    [string]$Destination,
    [hashtable]$Headers,
    $Asset
  )

  $tempDir = New-TemporaryDirectory
  try {
    $archivePath = Join-Path $tempDir $Asset.name
    Invoke-WebRequest -Uri $Asset.browser_download_url -Headers $Headers -OutFile $archivePath -ErrorAction Stop

    Expand-Archive -Path $archivePath -DestinationPath $tempDir -Force -ErrorAction Stop

    $binaryPath = Join-Path $tempDir "shelltape.exe"
    if (-not (Test-Path -LiteralPath $binaryPath)) {
      throw "The downloaded archive did not contain shelltape.exe."
    }

    if (-not (Test-Path -LiteralPath $Destination)) {
      New-Item -ItemType Directory -Path $Destination -Force | Out-Null
    }

    $installPath = Join-Path $Destination "shelltape.exe"
    Move-Item -LiteralPath $binaryPath -Destination $installPath -Force

    Write-Host "Installed shelltape to $installPath"
    Ensure-PathContainsDestination -Destination $Destination
  }
  finally {
    if (Test-Path -LiteralPath $tempDir) {
      Remove-Item -LiteralPath $tempDir -Recurse -Force
    }
  }
}

Ensure-WindowsPlatform
Ensure-SupportedArchitecture

$owner = "CaddyGlow"
$repo = "shelltape"
$target = "x86_64-pc-windows-gnu"
$apiUrl = "https://api.github.com/repos/$owner/$repo"

$Token = Resolve-GitHubToken -ExplicitToken $Token
$headers = Get-GitHubHeaders -Token $Token

$releaseUrl = if ($Tag) { "$apiUrl/releases/tags/$Tag" } else { "$apiUrl/releases/latest" }
$release = Get-GitHubRelease -Url $releaseUrl -Headers $headers
$asset = Select-ReleaseAsset -Release $release -Target $target

Install-ShelltapeBinary -Destination $Destination -Headers $headers -Asset $asset
