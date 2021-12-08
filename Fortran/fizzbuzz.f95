program fizzbuzz
    integer(kind=8) :: i = 0
    character(len=20) :: s
    do while (i < huge(i))
        i = i + 1
        s = ""
        if (mod(i, 3) == 0) then
            s = s//"fizz"
        end if
        if (mod(i, 5) == 0) then
            s = s//"buzz"
        end if
        if (s == "") then
            print *, i
        else
            print *, s
        end if
    end do
end program fizzbuzz
