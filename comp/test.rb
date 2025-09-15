count = gets.chomp

def cut(dand)
  empty = true
  dand.each do |i|
    if i.odd?
     empty = false 
    end
  end


  evens = []
  odds  = []

  dand.each do |i|
    if i.even?
      evens << i
    else
      odds << i
    end
  end

  odds  = odds.sort
  evens = evens.sort
  rodds = odds.sort.reverse
  queue = []

  evens.each{|i| queue << i}
  while odds.size > 0
    if odds.size == rodds.size
      odds.each do |i|
        queue << odds[i]
        odds.shift
        rodds.shift
      end
    end
  end

 puts "queue: #{queue}"
  sum = 0
  if empty == true
    puts "0"
  else
    queue.each{|i| sum += i}
    puts sum
  end
end


args = []
count.to_i.times do
  _fields = gets.chomp
  dand = gets.chomp.split.map(&:to_i)
  args << dand
end

args.each{|i| cut(i)}
