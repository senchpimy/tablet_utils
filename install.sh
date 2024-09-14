# !/bin/bash

sudo cp ./set-permissions.service /etc/systemd/system/set-permissions.service
sudo systemctl enable set-permissions
sudo systemctl start set-permissions

