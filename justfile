# dev servers
serve:
    dx serve
serve-style:
    pnpx @tailwindcss/cli -i ./tailwind.css -o ./assets/GENERATED_tailwind.css --watch

# builds
build:
    pnpx @tailwindcss/cli -i ./tailwind.css -o ./assets/GENERATED_tailwind.css && dx bundle --platform web --ssg --release