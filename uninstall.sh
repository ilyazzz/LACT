#!/bin/sh
sudo systemctl disable --now lactd &&
sudo rm /usr/local/bin/lact-daemon &&
sudo rm /usr/local/bin/lact-gui &&
sudo rm /etc/systemd/system/lactd.service &&
sudo rm /usr/local/share/applications/lact.desktop &&
sudo rm /usr/share/icons/hicolor/192x192/apps/lact.png
