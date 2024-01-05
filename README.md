# converter-cli
Generalized file conversion tool.

To run
```bash
# Build first
docker build -t converter .

# Run anywhere
docker run --rm --volume "`pwd`:/usr/src/converter" --user `id -u`:`id -g` -it converter
```