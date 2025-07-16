files = %W[file1 file2 file3 file4 file5 file6 file7 file8 file9]
mirrors = %W[a.example.org b.example.org c.example.org]

require "net/http"
require "webmock"

WebMock.enable!

include(WebMock::API)

request_count = 0
err = {status: 502}
ok = {status: 200, body: "OK!"}
stub_request(:get, /.*\.example\.org/)
  .to_return(
    -> (r) {
      request_count += 1
      request_count % 4 == 0 ? err : ok
    }
  )

files.each do |file|
  mirror = mirrors.first
  uri = URI("http://#{mirror}/#{file}")
  puts("Requesting #{uri}")
  result = Net::HTTP.get_response(uri)
  if result.code == "200"
    puts("Success!")
  else
    puts("Error #{result.code}")
    mirrors.shift
    redo
  end
end
