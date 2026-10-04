; Inno Setup script for the CashFlow installer (Windows, per-user, no admin
; rights needed). Built by .github/workflows/release.yml:
;   iscc /DAppVersion=2.0.0 packaging\windows\CashFlow.iss
; Expects the release build at target\x86_64-pc-windows-msvc\release\cashflow.exe.

#ifndef AppVersion
  #define AppVersion "0.0.0"
#endif
#define AppName "CashFlow"
#define AppExe "CashFlow.exe"

[Setup]
; Never change the AppId: Windows uses it to recognize updates of the same app.
AppId={{6F3B2C1A-8D4E-4F7A-9B2C-5E1D0A7C3B91}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=Ferdinand Hecker
AppPublisherURL=https://github.com/HeckerFerdinand/CashFlow
AppSupportURL=https://github.com/HeckerFerdinand/CashFlow/releases
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
DisableDirPage=auto
; Installs for the current user only: no administrator rights required.
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=..\..\dist
OutputBaseFilename=CashFlow-Setup
SetupIconFile=..\..\assets\icon.ico
UninstallDisplayIcon={app}\{#AppExe}
UninstallDisplayName={#AppName}
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "german"; MessagesFile: "compiler:Languages\German.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "..\..\target\x86_64-pc-windows-msvc\release\cashflow.exe"; DestDir: "{app}"; DestName: "{#AppExe}"; Flags: ignoreversion
Source: "..\..\CHANGELOG.md"; DestDir: "{app}"; DestName: "Changelog.txt"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExe}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExe}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent
