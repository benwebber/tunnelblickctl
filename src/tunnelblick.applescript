# vim: set ft=applescript :

on join(lst, delimiter)
  set buf to ""
  set currentDelimiters to AppleScript's text item delimiters
  set AppleScript's text item delimiters to delimiter
  set buf to lst as string
  set AppleScript's text item delimiters to currentDelimiters
  buf
end

script Tunnelblick

  to assertRunning()
    if not isRunning()
      error "Tunnelblick is not running"
    end
  end

  to connect(tunnel)
    assertRunning()
    tell application "Tunnelblick"
      connect tunnel
      get state of first configuration where name = tunnel
      repeat until result = "CONNECTED"
        delay 1
        get state of first configuration where name = tunnel
      end repeat
    end tell
  end

  to connectAll()
    assertRunning()
    tell application "Tunnelblick" to connect all
  end

  to disconnect(tunnel)
    assertRunning()
    tell application "Tunnelblick" to disconnect tunnel
    return
  end

  to disconnectAll()
    assertRunning()
    tell application "Tunnelblick" to disconnect all
  end

  to getConfigurations()
    assertRunning()
    tell application "Tunnelblick"
      my join((get name of configurations), "\n")
    end tell
  end

  to getStatus()
    assertRunning()
    set RECORD_SEPARATOR to character id 30
    set UNIT_SEPARATOR to character id 31
    set buf to {join({"name", "state", "autoconnect", "bytesout", "bytesin"}, UNIT_SEPARATOR)}
    tell application "Tunnelblick"
      repeat with n in (get name of configurations)
        set cfg to a reference to the first configuration where name = n
        copy my join({(get name of cfg), ¬
                      (get state of cfg), ¬
                      (get autoconnect of cfg), ¬
                      (get bytesOut of cfg), ¬
                      (get bytesIn of cfg)}, UNIT_SEPARATOR) to the end of buf
      end repeat
    end tell
    join(buf, RECORD_SEPARATOR)
  end

  to isRunning()
    application "Tunnelblick" is running
  end

  to launch()
    launch application "Tunnelblick"
  end

  to run
    run application "Tunnelblick"
  end

  to quit()
    assertRunning()
    tell application "Tunnelblick" to quit
  end

  to getVersion()
    get version of application "Tunnelblick"
  end

end
