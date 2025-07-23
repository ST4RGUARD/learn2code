# we dont care about this arg
def request(method, *)
  super
  method = method || "HTTP"
end
