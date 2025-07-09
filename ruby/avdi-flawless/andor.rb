nil || 42
42 && 23
nil or 42
42 and 23

user = Struct.new(:name).new("Avdi")
user_name = user && user.name
(user_name = user) and user.name

:x || :y && nil
:x or :y and nil

#line = $stdin.gets or raise("can't read from stdin") # =>

require "stringio"

chunked_data = StringIO.new("7\r\nHello, \r\n6\r\nworld!\r\n0\r\n")
data = ""
until chunked_data.eof?
  chunk_head = chunked_data.gets("\r\n") and
    chunk = chunked_data.read(chunk_head.to_i) and
    data <<
    chunk
end

data
