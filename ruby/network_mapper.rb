#!/usr/bin/env ruby
require 'nokogiri'
require 'optparse'
require 'csv'

options = {
  target: nil,
  ports: 'top100',
  csv: nil
}

OptionParser.new do |opts|
  opts.banner = 'Usage: sudo ruby network_mapper.rb [options]'

  opts.on('-t', '--target CIDR/IP', 'Target IP range (e.g. 10.36.128.0/21)') do |t|
    options[:target] = t
  end

  opts.on('-p', '--ports TYPE',
          "Port profile: 'fast' (top 20), 'top100' (default), or 'common' (22,80,443,445,3389)") do |p|
    options[:ports] = p
  end

  opts.on('-c', '--csv FILE', 'Export results to a CSV file') do |c|
    options[:csv] = c
  end

  opts.on('-h', '--help', 'Prints this help') do
    puts opts
    exit
  end
end.parse!

if options[:target].nil?
  puts '[!] Error: Target IP or CIDR range required (-t 10.36.128.0/21)'
  exit 1
end

# Ensure Nokogiri is available
begin
  require 'nokogiri'
rescue LoadError
  puts "[!] Missing required gem 'nokogiri'. Install it via: gem install nokogiri"
  exit 1
end

# Configure port selection string
port_flag = case options[:ports]
            when 'fast' then '--top-ports 20'
            when 'common' then '-p 22,80,443,445,3389,8080'
            else '--top-ports 100'
            end

xml_tmpfile = "/tmp/nmap_scan_#{Time.now.to_i}.xml"

puts "[+] Scanning target: #{options[:target]}..."
puts "[+] Running Nmap OS/Service/Host discovery (saving to #{xml_tmpfile})..."

# Command breakdown:
# -sS: Fast TCP SYN Stealth scan
# -sV: Probe open ports for service/version info
# --script: Runs NetBIOS & mDNS scripts for discovery of hostnames
# -oX: Output directly to structured XML
nmap_cmd = "sudo nmap -sS -sV #{port_flag} --script nbstat,dns-service-discovery #{options[:target]} -oX #{xml_tmpfile}"
system(nmap_cmd)

unless File.exist?(xml_tmpfile)
  puts '[!] Scan failed or XML file was not generated.'
  exit 1
end

doc = Nokogiri::XML(File.read(xml_tmpfile))
results = []

doc.xpath('//host').each do |host|
  # Skip hosts that are down
  next unless host.xpath('status/@state').text == 'up'

  # Extract IP Address
  ip = host.xpath('address[@addrtype="ipv4"]/@addr').text

  # Extract MAC & Vendor if available
  mac_elem = host.xpath('address[@addrtype="mac"]')
  mac = mac_elem.xpath('@addr').text
  vendor = mac_elem.xpath('@vendor').text
  mac_info = if mac.empty?
               'N/A'
             else
               "#{mac} (#{vendor.empty? ? 'Unknown' : vendor})"
             end

  # Extract Hostnames (DNS, NetBIOS, mDNS)
  hostnames = host.xpath('hostnames/hostname/@name').map(&:text)

  # Check NetBIOS script output if reverse DNS found nothing
  nb_name = host.xpath('hostscript/script[@id="nbstat"]/@output').text.match(/NetBIOS name: ([^,]+)/)
  hostnames << nb_name[1] if nb_name

  hostname = hostnames.uniq.reject(&:empty?).join(', ')
  hostname = 'Unknown' if hostname.empty?

  # Extract Open Ports and Services
  open_ports = []
  host.xpath('ports/port[state/@state="open"]').each do |port|
    port_id = port.attr('portid')
    proto   = port.attr('protocol')
    service = port.xpath('service/@name').text
    product = port.xpath('service/@product').text
    ver     = port.xpath('service/@version').text

    service_desc = service
    service_desc += " (#{product} #{ver})".strip unless product.empty?

    open_ports << "#{port_id}/#{proto} [#{service_desc}]"
  end

  ports_summary = open_ports.empty? ? 'No open ports found (in scanned range)' : open_ports.join('; ')

  results << {
    ip: ip,
    hostname: hostname,
    mac_vendor: mac_info,
    ports: ports_summary
  }
end

# Clean up temp file
File.delete(xml_tmpfile) if File.exist?(xml_tmpfile)

# --- Print Results to Console ---
puts "\n" + ('=' * 100)
puts format('%-16s | %-30s | %-45s', 'IP ADDRESS', 'HOSTNAME', 'OPEN PORTS & SERVICES')
puts '=' * 100

results.each do |r|
  puts format('%-16s | %-30s | %-45s', r[:ip], r[:hostname][0..29], r[:ports])
  puts format('%-16s | %-30s | %-45s', '', "MAC: #{r[:mac_vendor]}", '') unless r[:mac_vendor] == 'N/A'
  puts '-' * 100
end

# --- Export to CSV if requested ---
if options[:csv]
  CSV.open(options[:csv], 'w') do |csv|
    csv << ['IP Address', 'Hostname', 'MAC & Vendor', 'Open Ports & Services']
    results.each do |r|
      csv << [r[:ip], r[:hostname], r[:mac_vendor], r[:ports]]
    end
  end
  puts "\n[+] Results successfully exported to #{options[:csv]}"
end
