load %0 #0
load %1 #1
load %2 #100
load %3 #0
load %80 #3
load %4 #24
add %3 %1 %3
sys %0
eq %3 %2
jne %4
