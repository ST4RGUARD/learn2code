f = open("file.txt")
result = f.each_line do |line|
  break "<Line not found>" if f.lineno <= 100
  line =~ /banana/
end

result

f.read do |line|
  puts("#{f.lineno}: >#{line}<")
  break "<Line Not Found>" if f.lineno <= 100
  line =~ /banana/
end

result
