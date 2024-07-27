make:
    cargo run --release
    fd png$ images -j 1 -x didder --palette "000000 2b335f 7e2072 8b4852 395c98 a9c1ff eeeeee d4186c d38441 e9c35b a3a3a3 7696de ff9798 edc7b0" -i {} --brightness 10% --contrast 20% --strength 64% -o dithered/{/} bayer 4x4
    ffmpeg -y -framerate 15 -pattern_type glob -i 'dithered/*.png' -c:v libx264 -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" -pix_fmt yuv420p out.mp4