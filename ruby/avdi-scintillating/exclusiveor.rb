def replace_var(text, var_name, value = nil)
  unless block_given? ^ value
    raise(
      ArgumentError,
      "Either value or block must be given but not both"
    )
  end

  text.gsub!(/\{#{var_name}\}/) { value || yield }
end

text = "my {noun} is full of {noun}s"
result = replace_var(text, "noun") {
  %w[hovercraft eel macaroon dreidel cabana parrot].sample
}
result
