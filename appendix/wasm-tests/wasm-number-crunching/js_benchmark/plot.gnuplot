set terminal svg
set output 'boxplot.svg'

set border 2 front lt black linewidth 1.000 dashtype solid
set boxwidth 0.5 absolute
set style fill   solid 0.50 border lt -1
unset key
set pointsize 0.5
set style data boxplot
set xtics border in scale 0,0 nomirror norotate  autojustify
set xtics  norangelimit 
set xtics   ("A" 1.00000, "B" 2.00000)
set ytics border in scale 1,0.5 nomirror norotate  autojustify
set xrange [ * : * ] noreverse writeback
set x2range [ * : * ] noreverse writeback
set yrange [ 0.00000 : 100.000 ] noreverse nowriteback
set y2range [ * : * ] noreverse writeback
set zrange [ * : * ] noreverse writeback
set cbrange [ * : * ] noreverse writeback
set rrange [ * : * ] noreverse writeback

# plot "add_modulo.data"