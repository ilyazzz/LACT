#!/bin/sh
cargo build --release &&
sudo install -Dm755 target/release/daemon /usr/local/bin/lact-daemon &&
sudo install -Dm755 target/release/gui /usr/local/bin/lact-gui &&
sudo install -Dm755 target/release/cli /usr/local/bin/lact-cli && 
sudo install -Dm644 lactd.service /etc/systemd/system/lactd.service && 
sudo mkdir -p /usr/local/share/applications && 
sudo install -Dm644 lact.desktop /usr/local/share/applications/ && 
sudo mkdir -p /usr/share/icons/hicolor/192x192/apps &&
sudo install -Dm644 icons/lact.png /usr/share/icons/hicolor/192x192/apps/ &&
sudo systemctl daemon-reload 
sudo systemctl enable lactd &&
sudo systemctl restart lactd
