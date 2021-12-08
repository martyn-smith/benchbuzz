# naive implementation
for i = 1:typemax(UInt64)
    str = ""
    if i % 3 == 0
        str = str * "fizz"
    end
    if i % 5 == 0
        str = str * "buzz"
    end
    if str == ""
        str = string(i)
    end
    println(str)
end
