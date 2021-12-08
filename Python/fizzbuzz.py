for i in range(0, 2**32-1):
    s = []
    if i % 3 == 0:
        s += ["fizz"]
    if i % 5 == 0:
        s += ["buzz"]
    if s == []:
        s += [str(i)]
    print("".join(s))
