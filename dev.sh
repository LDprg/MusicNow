#! /usr/bin/bash
npx @tailwindcss/cli -i tailwind.css -o assets/tailwind.css --watch &
dx serve --platform web

wait
