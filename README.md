# converter-cli
Generalized file conversion CLI. Support video, image, text, audio and custom support.

## To run
```bash
# Build first
docker build -t converter .

# Run!
docker run --rm --volume "`pwd`:/data" --user `id -u`:`id -g` -it converter

# Or add to your .*sh file.
alias converter='docker run --rm --volume "`pwd`:/data" --user `id -u`:`id -g` -it converter'
```