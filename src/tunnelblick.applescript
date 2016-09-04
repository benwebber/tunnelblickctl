script Tunnelblick

  to connect(tunnel)
    tell application "Tunnelblick"
      connect tunnel
      get state of first configuration where name = tunnel
      repeat until result = "CONNECTED"
        delay 1
        get state of first configuration where name = tunnel
      end repeat
    end tell
  end

  to disconnect(tunnel)
    tell application "Tunnelblick"
      disconnect tunnel
    end tell
  end

  to listTunnels()
    set configs to ""
    tell application "Tunnelblick"
      repeat with name in (get name of configurations)
        set configs to configs & name & "\n"
      end repeat
    end tell
    return configs
  end

  to showStatus()
    set status to ""
    tell application "Tunnelblick"
      repeat with c in (get name of configurations)
        set currentConfig to a reference to the first configuration where name = c
        set _name to (get name of currentConfig)
        set _state to (get state of currentConfig)
        set _autoconnect to (get autoconnect of currentConfig)
        set _rx to (get bytesIn of currentConfig)
        set _tx to (get bytesOut of currentConfig)
        set status to status & _name & "\t" & _state & "\t" & _autoconnect & "\t" & _tx & "\t" & _rx & "\n"
      end repeat
    end tell
    return status
  end

  to launch()
    launch application "Tunnelblick"
  end

  to run
    run application "Tunnelblick"
  end

  to quit()
    tell application "Tunnelblick"
      quit
    end tell
  end

end
