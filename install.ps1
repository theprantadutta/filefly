param(
    [switch]
    $AllUsers
)

# Instructions for unsupported operating systems
$installInstructions = @'
Hey friend,

This installer is only available for Windows.
If you're looking for installation instructions for your operating system,
please visit the following link:
'@

if ($IsMacOS) {
    Write-Host @"
$installInstructions

https://yourlink.com/macos-installation
"@
    exit
}
if ($IsLinux) {
    Write-Host @"
$installInstructions

https://yourlink.com/linux-installation
"@
    exit
}

# Determine the system architecture
$installer = ''
$arch = (Get-CimInstance -Class Win32_Processor -Property Architecture).Architecture | Select-Object -First 1
switch ($arch) {
    0 { $installer = "filefly_windows_x86.exe" } # x86
    5 { $installer = "filefly_windows_arm64.exe" } # ARM
    9 {
        if ([Environment]::Is64BitOperatingSystem) {
            $installer = "filefly_windows_amd64.exe"
        }
        else {
            $installer = "filefly_windows_x86.exe"
        }
    }
    12 { $installer = "filefly_windows_arm64.exe" } # Surface Pro X
}

if ([string]::IsNullOrEmpty($installer)) {
    Write-Host @"
The installer for system architecture ($arch) is not available.
"@
    exit
}

Write-Host "Downloading $installer..."

# Validate the availability of New-TemporaryFile
if (Get-Command -Name New-TemporaryFile -ErrorAction SilentlyContinue) {
    $tmp = New-TemporaryFile | Rename-Item -NewName { $_ -replace 'tmp$', 'exe' } -PassThru
} else {
    $tmp = New-Item -Path $env:TEMP -Name ([System.IO.Path]::GetRandomFileName() -replace '\.\w+$', '.exe') -Force -ItemType File
}

# Construct the download URL
$url = "https://github.com/theprantadutta/filefly/releases/latest/download/$installer"

# Check if we can make HTTPS requests and download the binary
try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $url -Method Head | Where-Object { $_.StatusCode -ne 200 }  # Suppress success output
} catch {
    Write-Host "Unable to download $installer. Please check your internet connection."
    exit
}

# Download the installer
Invoke-WebRequest -OutFile $tmp $url
Write-Host 'Running installer...'

# Define the installation path
$installDir = "$HOME\AppData\Local\Programs\filefly"

# Create installation directory if it doesn't exist
if (-Not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir
}

# Move the downloaded executable to the installation directory
Move-Item -Path $tmp -Destination "$installDir\filefly.exe" -Force

# Add to user's PATH permanently
$envPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if (-not $envPath.Split(";").Contains($installDir)) {
    [Environment]::SetEnvironmentVariable("Path", "$envPath;$installDir", [EnvironmentVariableTarget]::User)
    Write-Host "Added $installDir to the user's PATH."
} else {
    Write-Host "$installDir is already in the user's PATH."
}

Write-Host "Filefly v1.0.0 installed successfully in $installDir!"
Write-Host @'
Done!

Restart your terminal and have a look at the documentation on how to proceed from here.

https://yourlink.com/documentation
'@
