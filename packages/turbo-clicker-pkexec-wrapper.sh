#!/bin/bash

pkexec env WAYLAND_DISPLAY="$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" XDG_RUNTIME_DIR="/run/user/0" /usr/bin/turbo-clicker
