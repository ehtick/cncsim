#!/usr/bin/env bash

file='/home/cameron/Downloads/ManhattanH-5.nc'
#file='/home/cameron/Projects/rust/kahm_cam/HAAS-1.nc'
#file='/home/cameron/Downloads/VisaREMAKE-2.nc'
#file='/home/cameron/Downloads/Marys.nc'


runcustomblockwidth() {
    cargo run -- --input "$file" --output image.png --blockwidth "$1" --blockheight "$2" --imgwidth $(($1 * $3)) --imgheight $(($2 * $3)) --fnvalue 100 && (xdg-open image.png &);# (sleep 2 && clear)
}

echo 'cargo run --release -- --input '"$file"' --output image.png --blockwidth 20 --blockheight 20 --imgwidth 4096 --imgheight 4096 --fnvalue 100 && xdg-open image.png'
#cargo run -- --input "$file" --output image.png --blockwidth 20 --blockheight 10 --imgwidth 4096 --imgheight 2048 --fnvalue 100 && xdg-open image.png
#runcustomblockwidth 13 20 "$((2**7))"
#runcustomblockwidth 13 20 "$((2**6))"
runcustomblockwidth 15 12 "$((2**8))"

