#! /usr/bin/bash
tailwindcss -i tailwind.css -o assets/tailwind.css --watch &
dx serve --platform web

wait
