#!/usr/bin/env bash
# Name:        pre_uninstall.sh
# Description: Distro-agnostic pre-uninstallation script for sysmqttd
# Author:      Nicholas Wilde <https://github.com/nicholaswilde/>
# Date:        2025-01-01
# Version:     0.1.0
set -e
set -o pipefail

main() {
  # Detect if this is an upgrade
  IS_UPGRADE=false
  if [ "$1" = "upgrade" ]; then
      IS_UPGRADE=true
  elif [ "$1" = "1" ]; then
      IS_UPGRADE=true
  fi

  # Stop and disable the service if systemd is running
  if [ -d /run/systemd/system ]; then
      if [ "$IS_UPGRADE" = "true" ]; then
          # During upgrade, only stop the service to avoid "text file busy" when unpacking,
          # but do NOT disable it.
          systemctl stop sysmqttd.service >/dev/null 2>&1 || :
      else
          # Full uninstall: stop and disable
          systemctl stop sysmqttd.service >/dev/null 2>&1 || :
          systemctl disable sysmqttd.service >/dev/null 2>&1 || :
      fi
  fi

  exit 0
}

main "$@"
