!define MUI_PAGE_CUSTOMFUNCTION_LEAVE RetargetFinishPageShortcuts

Function RetargetFinishPageShortcuts
  ; Ensure Desktop shortcut targets launcher-host.exe with --open and working directory $INSTDIR
  SetOutPath "$INSTDIR"
  IfFileExists "$DESKTOP\Universal launcher.lnk" 0 +2
    CreateShortcut "$DESKTOP\Universal launcher.lnk" "$INSTDIR\launcher-host.exe" "--open" "$INSTDIR\icons\icon.ico" 0
FunctionEnd

!macro NSIS_HOOK_PREINSTALL
  ; Gracefully signal launcher-host to quit via IDM_QUIT (2002)
  FindWindow $0 "UniversalLauncherHostClass" "UniversalLauncherHost"
  IntCmp $0 0 +3
  SendMessage $0 0x0111 2002 0 /TIMEOUT=3000
  Sleep 500

  ; Gracefully signal standalone universal-launcher UI to close if running
  FindWindow $0 "Tauri Window" "Universal launcher"
  IntCmp $0 0 +3
  SendMessage $0 0x0010 0 0 /TIMEOUT=3000
  Sleep 500
!macroend

!macro NSIS_HOOK_POSTINSTALL
  ; 1. Ensure launcher-host.exe exists at $INSTDIR root
  IfFileExists "$INSTDIR\resources\launcher-host.exe" 0 +2
    CopyFiles /SILENT "$INSTDIR\resources\launcher-host.exe" "$INSTDIR\launcher-host.exe"

  ; 2. Ensure icons directory and icon.ico exist at $INSTDIR\icons\icon.ico
  CreateDirectory "$INSTDIR\icons"
  IfFileExists "$INSTDIR\resources\icons\icon.ico" 0 +2
    CopyFiles /SILENT "$INSTDIR\resources\icons\icon.ico" "$INSTDIR\icons\icon.ico"
  IfFileExists "$INSTDIR\resources\icon.ico" 0 +2
    CopyFiles /SILENT "$INSTDIR\resources\icon.ico" "$INSTDIR\icons\icon.ico"

  ; 3. Explicitly set working directory for shortcuts to $INSTDIR
  SetOutPath "$INSTDIR"

  ; 4. Create Start Menu shortcuts targeting launcher-host.exe with --open
  CreateDirectory "$SMPROGRAMS\Universal launcher"
  CreateShortcut "$SMPROGRAMS\Universal launcher\Universal launcher.lnk" "$INSTDIR\launcher-host.exe" "--open" "$INSTDIR\icons\icon.ico" 0
  CreateShortcut "$SMPROGRAMS\Universal launcher.lnk" "$INSTDIR\launcher-host.exe" "--open" "$INSTDIR\icons\icon.ico" 0

  ; 5. Create Desktop shortcut targeting launcher-host.exe with --open
  CreateShortcut "$DESKTOP\Universal launcher.lnk" "$INSTDIR\launcher-host.exe" "--open" "$INSTDIR\icons\icon.ico" 0
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ; Gracefully signal launcher-host to quit via IDM_QUIT (2002)
  FindWindow $0 "UniversalLauncherHostClass" "UniversalLauncherHost"
  IntCmp $0 0 +3
  SendMessage $0 0x0111 2002 0 /TIMEOUT=3000
  Sleep 500

  ; Gracefully signal standalone universal-launcher UI to close if running
  FindWindow $0 "Tauri Window" "Universal launcher"
  IntCmp $0 0 +3
  SendMessage $0 0x0010 0 0 /TIMEOUT=3000
  Sleep 500
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  ; Remove secondary binaries, icons folder, and residual shortcuts
  Delete "$INSTDIR\launcher-host.exe"
  RMDir /r "$INSTDIR\icons"
  Delete "$SMPROGRAMS\Universal launcher.lnk"
  Delete "$SMPROGRAMS\Universal launcher\Universal launcher.lnk"
  RMDir "$SMPROGRAMS\Universal launcher"
  Delete "$DESKTOP\Universal launcher.lnk"
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "UniversalLauncher"
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Universal launcher"

  ; Remove transient WebView2 runtime cache
  RMDir /r "$LOCALAPPDATA\com.atom.universal-launcher\EBWebView"
  RMDir "$LOCALAPPDATA\com.atom.universal-launcher"
!macroend
