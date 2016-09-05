on join(lst, delimiter)
  set buf to ""
  set currentDelimiters to AppleScript's text item delimiters
  set AppleScript's text item delimiters to delimiter
  set buf to lst as string
  set AppleScript's text item delimiters to currentDelimiters
  buf
end

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
    tell application "Tunnelblick" to disconnect tunnel
    return
  end

  to listTunnels()
    tell application "Tunnelblick"
      my join((get name of configurations), "\n")
    end tell
  end

  to showStatus()
    set buf to {join({"NAME", "STATE", "AUTOCONNECT", "TX", "RX"}, "\t")}
    tell application "Tunnelblick"
      repeat with n in (get name of configurations)
        set cfg to a reference to the first configuration where name = n
        copy my join({(get name of cfg), ¬
                      (get state of cfg), ¬
                      (get autoconnect of cfg), ¬
                      (get bytesOut of cfg), ¬
                      (get bytesIn of cfg)}, "\t") to the end of buf
      end repeat
    end tell
    join(buf, "\n")
  end

  to launch()
    launch application "Tunnelblick"
  end

  to run
    run application "Tunnelblick"
  end

  to quit()
    tell application "Tunnelblick" to quit
    return
  end

  to getVersion()
    get version of application "Tunnelblick"
  end

end
