# Match Digits

titles = [
  "test123-string",
  "564-another",
  "stat-458",
  "156",
  "888-555-hello"
]

PATTERN = /
  (?<!\d)         # preceeded by non-digits
  \d{3}           # exactly three digits in a row # =>
  (?!\d)          # followed by non-digits
/x

titles.each do |i|
  puts("#{PATTERN.match(i)}")
end
