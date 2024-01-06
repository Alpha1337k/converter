# converter
Convert files from the CLI. Supports audio, images, text and video, with the ability to add your own easily!

## To run
```bash
# Build first
docker build -t converter .

# Run!
docker run --rm --volume "`pwd`:/data" --user `id -u`:`id -g` -it converter

# Or add to your .*sh file.
alias converter='docker run --rm --volume "`pwd`:/data" --user `id -u`:`id -g` -it converter'
```

## Bugs, features.
Please use the tools GitHub provides to contact me. Pull requests and issues / requests are always welcome!