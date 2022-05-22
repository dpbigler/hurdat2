## HURDAT2

### Setup (tl;dr)
Install [Cargo](https://www.rust-lang.org/tools/install) through rustup. Then,
```sh
cargo install --git https://github.com/dpbigler/hurdat2.git
```
will install the command line tool `hurdat2`, which can be run with
```
hurdat2 <hurdat2 file location> <start year> <end year>
```
e.g.,
```
> hurdat2 <hurdat2 file> 2000 2010
| Name            | Landfall                  | Max Sustained (kt)   | Max Gust (kt)   |
----------------------------------------------------------------------------------------
| GORDON          | 2000-09-18 06:00:00 UTC   |                   40 |              62 |
----------------------------------------------------------------------------------------
| HELENE          | 2000-09-22 12:00:00 UTC   |                   35 |              54 |
----------------------------------------------------------------------------------------
| LESLIE          | 2000-10-04 12:00:00 UTC   |                   30 |              46 |
...

```

### Details
This application attempts to construct an analysis of the hurdat2
dataset provided by the National Hurricane Center. In particular,
we print a table consisting of all hurricanes that landed in Florida 
during the specified years, an estimate of the date and time that landfall 
occurred, and an estimate of the maximum sustained and gust wind speeds.

We use cartographic boundary files, in KML format, provided by the US
Census bureau for landfall estimation. We use the Hurricane's 
last known location outside of Florida and the first known location inside
of Florida, as well as the times at which they measured, to 




### Some Benchmarking
```
> du -sh .cargo/bin/hurdat2 
4.2M	.cargo/bin/hurdat2
```
```
time hurdat2 data/hurdat2-1851-2021-041922.txt 1900 2010
...
real	0m0.088s
user	0m0.566s
sys	0m0.322s
```

### Data
Hurdat2 --

KML file source
https://www.census.gov/geographies/mapping-files/time-series/geo/cartographic-boundary.html
20M:1 resolution KML files
