#!/usr/bin/python

import pefile,pprint

print "./script path/to/file"

# not pretty just has to work!
file=sys.argv[1]
pe=pefile.PE(file)

for dll in pe.DIRECTORY_ENTRY_IMPORT:
    print '[+] Imported DLLs [+]'
    print '[+] ' , entry.dll, ' [+]'
