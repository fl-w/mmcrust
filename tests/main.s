# fn begin main 0
# count = 0
    li $count, 0        # label W1
    W1:                 # if x>=y goto W2 
    bge x, y, W2        # t0 = 2 * count
# count = t0
    add count, $R0, t0  # t1 = x + 1
# x = t1
    add x, $R0, t1      # goto W2
    j W2                # goto W1
    j W1                # label W2
    W2:                 # end(main)
# fn begin plus 2
# t2 = a + b
    lw                  # end(plus)
# fn begin do 3
# end(do)
# fn begin main 1
# x = 2
    li $x, 2            # y = 3
    li $y, 3            # fn begin times 2
# t3 = a * b
    lw                  # end(times)
# end(main)
# fn begin fact 1
# fn begin inner_fact 2
# if n!=0 goto L1 
    bne n, 0, L1        # label L1
    L1:                 # t4 = n - 1
    sub t4, n, 1        # t5 = a * n
    lw                  # end(inner_fact)
# end(fact)
# fn begin min 2
# if a>=b goto L2 
    bge a, b, L2        # goto L2
    j L2                # label L2
    L2:                 # label L2
    L2:                 # end(min)
# fn begin cplus 1
# fn begin cplusa 1
# t6 = a + b
    lw                  # end(cplusa)
# end(cplus)
# fn begin twice 1
# fn begin g 1
# end(g)
# end(twice)
