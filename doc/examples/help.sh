#!/bin/sh

show_help()
{
    echo "Usage: example.sh OPTIONS <input-file...>"
    echo "  -l, --long           enable detailed output"
    echo "  -o, --out-file FILE  file to write result"
}

show_version()
{
    echo "help.sh 1.0"
}


eval "$(parseargs -n help.sh.sh -hv -o 'l:long#detailed,o:out-file=outfile' -- "$@")" || exit 1

