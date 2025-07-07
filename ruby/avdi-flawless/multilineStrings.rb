# multiline strings
#
wocky = "Twas brillig, and the slithy toves
did gyre and gimble in the wabe;
all mimsy were the borogoves,
and the mome raths outgrabe.

\"beware the jabberwock, my son\""

wocky2 = <<UNIQUE_STRING
  Twas brillig, and the slithy toves
did gyre and gimble in the wabe;
all mimsy were the borogoves,
and the mome raths outgrabe.

"beware the jabberwock, my son"}
UNIQUE_STRING
  .upcase

wocky3 = <<-UNIQUE_STRING
  Twas brillig, and the slithy toves
  did gyre and gimble in the wabe;
  all mimsy were the borogoves,
  and the mome raths outgrabe.

  "beware the jabberwock, my son"}
UNIQUE_STRING
  .upcase

# - preserve indentation
# ~ strip indentation
module Wonderland
  JABBERWOCKY = <<~UNIQUE_STRING
    Twas brillig, and the slithy toves
    did gyre and gimble in the wabe;
    all mimsy were the borogoves,
    and the mome raths outgrabe.

    "beware the jabberwock, my son"}
  UNIQUE_STRING
    .upcase
end
# => nil
puts(Wonderland::JABBERWOCKY)

wocky
wocky2
wocky3

# >> TWAS BRILLIG, AND THE SLITHY TOVES
# >> DID GYRE AND GIMBLE IN THE WABE;
# >> ALL MIMSY WERE THE BOROGOVES,
# >> AND THE MOME RATHS OUTGRABE.
# >>
# >> "BEWARE THE JABBERWOCK, MY SON"}
