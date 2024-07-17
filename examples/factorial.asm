load %0 #0
load %1 #1
load %2 #6
load %80 #1
load %3 #20
mul %1 %1 %2
sub %2 %2 %80
eq %2 %80
jne %3
sys %0
