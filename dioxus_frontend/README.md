# Dioxus Frontend

The frontend part.

## Development

You are expected to be running in a Nix development shell.

To start the Tailwind compiler in watch mode, run:

```bash
tailwindcss -i ./css/input.css -o ./assets/tailwind.css --watch
```

Run the following command to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

You can now open a browser to http://localhost:8080 and see your app!
