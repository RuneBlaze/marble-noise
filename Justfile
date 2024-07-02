make:
    cargo run --release
    fd png$ images -j 1 -x didder --palette "000000 2b335f 7e2072 19959c 8b4852 395c98 a9c1ff eeeeee d4186c d38441 e9c35b a3a3a3 70c6a9 7696de ff9798 edc7b0" -i {} -o dithered/{/} --strength 64% bayer 8x8
    ffmpeg -framerate 15 -pattern_type glob -i 'dithered/*.png' -c:v libx264 -pix_fmt yuv420p out.mp4