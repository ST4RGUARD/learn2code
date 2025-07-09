step_count = 10
spaces_count = 10
steps = []
spaces = []

step_count.times do
  spaces_count.times do
    spaces << " "
  end

  steps << "#"
  puts("#{spaces.join}#{steps.join}")

  spaces_count -= 1
  spaces = []
end

bump_cnt = 10
stair = ""
bump_cnt.times do
  stair += "#"
  puts(stair)
end
