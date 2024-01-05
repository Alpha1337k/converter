# converter-cli
Generalized file conversion CLI. Support video, text, audio and custom support.

## To run
```bash
# Build first
docker build -t converter .

# Run!
docker run --rm --volume "`pwd`:/usr/src/converter" --user `id -u`:`id -g` -it converter
```