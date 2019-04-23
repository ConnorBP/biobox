load $0 #21
load $1 #42
div $1 $0 $2
load $3 #1
eq $2 $3
load $8 #40
jeq $8
nop
nop
nop
load $21 #420
dec $1
gt $1 $0
jeq $31
load $6 #111