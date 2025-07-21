$tables_remaining = 10

class Request
  def host(new_host)
    @host = new_host
    self
  end

  def port(new_port)
    # return unless new_port.is_a?(Integer)
    # fail will not happen, ensure with return eats exceptions
    fail(TypeError) unless new_port.is_a?(Integer)

    @port = new_port
  ensure
    # no returns will matter if we use this, - ensure overrides
    return self
  end

  def path(new_path)
    @path = new_path
    self
  end

  def inspect
    "GET #{@host}#{@port && ":"}#{@port}#{@path}"
  end
end

def book_table(phone)
  $tables_remaining -= 1
  return unless valid_payment?
  return unless valid_sms?(phone)

  if success = create_reservation(phone)
    puts("Your table has been reserved")
  else
    puts("Sorry, there was a problem making the reservation")
  end

ensure
  $tables_remaining += 1 unless success
end

def valid_payment?
  true
end

def valid_sms?(sms)
  true
end

def create_reservation(sms)
  false
end

book_table("2232-32323")
$tables_remaining

Request.new.host("imahost.com").port(nil).path("/path")
