def fac(a)
 if a == 0
   return 1
 else
   return a * fac(a-1)
 end
end

puts fac(ARGV[0].to_i)
