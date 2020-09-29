set terminal svg enhanced font 'Verdana,20'

set style line 1 \
    linecolor rgb '#f0db4f' \
    linetype 1 linewidth 2.5 \
    pointtype 7 pointsize 1.5
set style line 2 \
    linecolor rgb '#f74c00' \
    linetype 1 linewidth 1.5 \
    pointtype 5 pointsize 0.75
set style line 3 \
    linecolor rgb '#000000' \
    linetype 0 linewidth 3 \
    pointtype 9 pointsize 0.5
set style line 4 \
    linecolor rgb '#808080' \
    linetype 1 linewidth 2 \
    pointtype 15 pointsize 0.75
set style line 5 \
    linecolor rgb '#1096cb' \
    linetype 1 linewidth 2 \
    pointtype 15 pointsize 0.75


set ylabel "Time to Complete [s]"

# set key above left vertical maxrows 2
# set key at 0,0 left vertical maxrows 2
set key above vertical maxrows 2

set log x
set log y
set xrange [100000:*]

set output 'small_number.svg'
set xlabel "Total Numbers Summed Up (1...)"
plot "lineplots.data" i  0 u 1:($2/1000)    title "JavaScript"  with linespoints ls 1,\
     "lineplots.data" i  2 u 1:($2/1000)    title "WASM"        with linespoints ls 2,\
     "lineplots.data" i  9 u 1:($2/1000000) title "AMD64 1x"    with linespoints ls 3,\
     "lineplots.data" i 12 u 1:($2/1000000) title "AMD64 12x"   with linespoints ls 4,\



set output 'medium_number.svg'
set xlabel "Total Numbers Summed Up (u32Max...)"
plot "lineplots.data" i  3 u 1:($2/1000)    title "JavaScript"  with linespoints ls 1,\
     "lineplots.data" i  5 u 1:($2/1000)    title "WASM"        with linespoints ls 2,\
     "lineplots.data" i 10 u 1:($2/1000000) title "AMD64 1x"    with linespoints ls 3,\
     "lineplots.data" i 13 u 1:($2/1000000) title "AMD64 12x"   with linespoints ls 4,\


set key above vertical maxrows 3
set output 'large_number.svg'
set xlabel "Total Numbers Summed Up (...jsIntMax)"
plot "lineplots.data" i  6 u 1:($2/1000)    title "JS number"   with linespoints ls 1,\
     "lineplots.data" i  7 u 1:($2/1000)    title "JS BigInt"   with linespoints ls 5,\
     "lineplots.data" i  8 u 1:($2/1000)    title "WASM"        with linespoints ls 2,\
     "lineplots.data" i 11 u 1:($2/1000000) title "AMD64 1x"    with linespoints ls 3,\
     "lineplots.data" i 14 u 1:($2/1000000) title "AMD64 12x"   with linespoints ls 4,\
