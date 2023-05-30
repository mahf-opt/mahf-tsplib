# MAHF TSPLIB

This crate provides a [MAHF](https://github.com/mahf-opt/mahf) problem instance for [TSPLIB](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/).

## Modifications

The current instances are from `2023-02-30` with the following modifications:

Renamed to lower case:
- kroA100.opt.tour
- kroA100.tsp
- kroA150.tsp
- kroA200.tsp
- kroB100.tsp
- kroB150.tsp
- kroB200.tsp
- kroC100.opt.tour
- kroC100.tsp
- kroD100.opt.tour
- kroD100.tsp
- kroE100.tsp

Expanded tour to one value per line:
- gr24.opt.tour
- gr48.opt.tour
- pr1002.opt.tour
- rd100.opt.tour
- ulysses16.opt.tour

Removed invalid type suffix:
- si175.tsp
- si535.tsp
- si1032.tsp
