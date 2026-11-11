; Instalador de Pixora (Inno Setup 6). Se compila con:
;   ISCC.exe /DAppVersion=1.0.0 /DSourceDir=..\dist-win /DOutputDir=..\out installer\pixora.iss
#ifndef AppVersion
  #error Falta /DAppVersion=X.Y.Z
#endif
#ifndef SourceDir
  #define SourceDir "..\dist-win"
#endif
#ifndef OutputDir
  #define OutputDir "..\out"
#endif

#define AppName "Pixora"
#define AppExe "pixora.exe"

[Setup]
; GUID fijo: NO cambiar entre versiones (permite actualizar/desinstalar la misma app).
AppId={{26DEAA9D-51A8-4F71-9893-5C2BD9D49F1D}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=Pixora
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
OutputDir={#OutputDir}
OutputBaseFilename=Pixora-Setup-{#AppVersion}-x64
SetupIconFile=..\src-tauri\icons\icon.ico
UninstallDisplayIcon={app}\{#AppExe}
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
; Sin telemetría ni red: el instalador solo descarga WebView2 si falta (ver [Code]).

[Languages]
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; EXE + DLL de libvips y dependencias (generado por scripts/package-windows.ps1)
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs; Excludes: "MicrosoftEdgeWebview2Setup.exe"
Source: "{#SourceDir}\MicrosoftEdgeWebview2Setup.exe"; DestDir: "{tmp}"; Flags: deleteafterinstall skipifsourcedoesntexist; Check: NeedsWebView2

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExe}"
Name: "{group}\Desinstalar {#AppName}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Run]
Filename: "{tmp}\MicrosoftEdgeWebview2Setup.exe"; Parameters: "/silent /install"; StatusMsg: "Instalando WebView2…"; Check: NeedsWebView2; Flags: waituntilterminated skipifdoesntexist
Filename: "{app}\{#AppExe}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent

[Code]
function WebView2Installed(): Boolean;
var
  v: String;
begin
  Result :=
    RegQueryStringValue(HKLM, 'SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}', 'pv', v) or
    RegQueryStringValue(HKCU, 'Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}', 'pv', v);
  Result := Result and (v <> '') and (v <> '0.0.0.0');
end;

function NeedsWebView2(): Boolean;
begin
  Result := not WebView2Installed();
end;
