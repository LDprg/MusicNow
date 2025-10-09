#! /usr/bin/bash
npx @tailwindcss/cli -i tailwind.css -o assets/tailwind.css --watch > /dev/null 2>&1 &
dx serve --platform web
