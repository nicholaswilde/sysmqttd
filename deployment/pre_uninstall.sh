#!/bin/sh
# Distro-agnostic pre-uninstallation script for sysmqttd
set -e

# Stop and disable the service if systemd is running
if [ -d /run/systemd/system ]; then
    # Disable and stop the service
    systemctl stop sysmqttd.service >/dev/null 2>&1 || :
    systemctl disable sysmqttd.service >/dev/null 2>&1 || :
fi

exit 0
