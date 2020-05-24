
# pygisO3

A library of GIS helper functions written in Rust, executable in Python.

__Note:  This is more a learning exercise for porting Rust to Python, than it is about developing good GIS modules.__

## Installation

Before installing please ensure you have rust installed!

1) Clone the repo
2) Create a python venv and install requirements.txt
3) Ensure that you are using rust-nightly:

   ```bash
   rustup override set nightly
   ```

4) To build locally run ```maturin develop```
5) Use!

## Examples

### Haversine

```python
from pygis_o3 import haversine

lat_from, lon_from, lat_to, lon_to = 123.1111232, 43.342432, 127.98293, 44.7863254
distance = haversine(lat_from, lon_from, lat_to, lon_to)
```
