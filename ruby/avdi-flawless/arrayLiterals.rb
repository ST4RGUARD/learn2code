width = 900
height = 600

record_opts = %W[-f x11grab -s #{width}x#{height} -i 0:0+800,300 -r 30]
misc_opts = %W[-sameq -threads 6]
output_opts = %W[cast.mp4]

mpeg_flags = record_opts +
  misc_opts +
  output_opts

system("ffmpeg", *mpeg_flags)
