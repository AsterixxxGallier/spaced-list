```
   5______,
   2__,3__,
1 A   B   C

node before 4 (expected result: ([1], 1)):
    current degree:   1
    current index:    0 (=A)
    current position: 0
    next index:       2 (=C)
    next position:    5
    
    overshoot
    
    current degree:   0
    current index:    0 (=A)
    current position: 0
    next index:       1 (=B)
    next position:    2
    
    undershoot
    
    move ahead unconditionally:
    current degree:   0
    current index:    1 (=B)
    current position: 2
    next index:       2 (=C)
    next position:    5
    
    current position < position < next position && degree == 0 && no sublists
    => done

newfound knowledge:
    find the smallest range from current_position to next_position, such that it (inclusively) contains target_position
    therefore, the range from current_index to next_index (inclusively) contains target_index (the unknown variable)
    
    current_position <= target_position <= next_position
    therefore, current_index <= target_index <= next_index
    
    if current_position == target_position:
        current_index == target_index
    if target_position == next_position:
        target_index == next_index
    
    if current_position <= target_position:
        current_index <= target_index
    
    if target_position <= next_position:
        target_index <= next_index
    
    if current_position < target_position:
        current_index < target_index
    if target_position < next_position:
        target_index < next_index
    
    node_before:
        if current_position < target_position:
            return (current_index, target_position - current_position)
        else /*if current_position == target_position)*/:
            return (previous_index, target_position - previous_position)
    
    node_before_or_at:
        return (current_index, target_position - current_position)
    
    node_at:
        if current_position == target_position:
            return current_index
        else if target_position == next_position:
            return next_index
    
    node_after_or_at:
        return (next_index, next_position - target_position)
    
    node_after:
        if target_position < next_position:
            return (next_index, next_position - target_position)
        else /*if target_position == next_position)*/:
            return (next_next_index, next_next_position - target_position)
```






























```
AE
AC     CE
AB BC  CD DE
A  B 3 C  D (E)
    |
    LM
    L M
0  12 34  5 (6)
  
3:  
A
A
B
B
-
L
/M
```

























```

000 001 010 100 1000
001 010
010 011 100
011 100
100 101 110 1000
101 110
110 111 1000
111 1000


000  0000 1000 1100 1110
001  0001
010  0010 1001
011  0011 
100  0100 1010 1101
101  0101 
110  0110 1011
111  0111 

```