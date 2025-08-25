alpha = ("a".."z")
ref = {}

alpha.each_with_index { |k, v| ref[k] = v }
str1 = gets.chomp.downcase
str2 = gets.chomp.downcase

new_s1 = []
new_s2 = []

str1.split("").each do |i|
  new_s1 << ref[i]
end

str2.split("").each do |i|
  new_s2 << ref[i]
end

equal = 0

new_s1.each_with_index do |val, i|
  if val == new_s2[i]
    equal = 1
  elsif (val < new_s2[i])
    equal = 0
    puts("-1")
    exit
  else
    equal = 0
    puts("1")
    exit
  end
end

if equal == 1
  puts("0")
end
