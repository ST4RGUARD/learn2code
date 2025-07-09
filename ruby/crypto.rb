require 'rest-client'
require 'json'
require 'pp'
require 'base64'

class CryptoAPI

  def initialize(email)
    @base = 'http://*c*r*y*p**to**.pr**aetor**i*an.*com'
    @email = email
    @token = self.token
  end

  def token
    data = {'email' => @email}.to_json
    ret = RestClient.post "#{@base}/api-token-auth/", data, :content_type => :json, :accept => :json
    JSON.parse(ret)['token']
  end

  def fetch(level)
    ret = RestClient.get "#{@base}/challenge/#{level}/", :content_type => :json, :accept => :json, :Authorization => "JWT #{@token}"
    JSON.parse(ret)
  end

  def solve(level, guess)
    data = {'guess' => guess}
    ret = RestClient.post "#{@base}/challenge/#{level}/", data, :content_type => :json, :accept => :json, :Authorization => "JWT #{@token}"
    JSON.parse(ret)
  end

  def status
    ret = RestClient.get "#{@base}/hash/", :content_type => :json, :accept => :json, :Authorization => "JWT #{@token}"
    JSON.parse(ret)
  end

end

# Class values
# Declare Class
crypto = CryptoAPI.new('blah')

# Fetch level 
level = 3
pp data = crypto.fetch(level)

# Caesar Cipher 3 - 4 Level 1
#-----------
#-----------
#guess = []

#data['challenge'].split(//).each do |char|
 # num = char.ord
  # -- needed to convert lowercase xyz manually bc they are converted to weird symbols
  #if num == 120 #x
    #num = 97
  #elsif num == 121 #y
    #num = 98
  #elsif num == 122 #z
    #num = 99
  #else
    #num += 3
  #end
   #char = num.chr
   #guess << char
#end

#pp guess = guess.join
#----------
#----------

# Base64 Level 2
# laughable i know
# guess = data['challenge'].split(',')
# guess = Base64.decode64(guess[1])
# was more code here but it magically disappeared
# guess = guess[-50..-17]

# pp guess

pp guess
pp crypto.solve(level, guess)
pp d = crypto.status

puts '*P*r*&aetor**ian C*ryp*t*o C**hal**lenge'
puts "  Level: #{d['level']}"
puts "  Hash: #{d['hash']}"
