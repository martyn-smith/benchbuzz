# naive implementation
1.upto (2**64 -1) do |i|
  str = ""
  if i % 3 == 0
    str += "fizz"
  end
  if i % 5 == 0
    str += "buzz"
  end
  if str == ""
    str += i.to_s
  end
  puts str
end
