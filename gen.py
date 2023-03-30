import random

with open("test.obj",'a') as f:
    i = 40.0
    while i < 60.0:
        r_number = random.gauss(0.5,0.1)
        if r_number >= 0.3 and r_number < 0.5:
            i += r_number
            i = round(i,2)
            f.write(f"v {i} 0.00 0.00\n")