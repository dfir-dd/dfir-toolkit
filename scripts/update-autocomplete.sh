#!/bin/bash

DIR=$( cd -- "$( dirname "${BASH_SOURCE[0]}" )"  && pwd)

mkdir -p $DIR/maintainer
echo '#!/bin/bash' >$DIR/maintainer/postinst

echo 'mkdir -p /usr/local/share/zsh/site-functions' >>$DIR/maintainer/postinst

for B in $(cd src/bin; echo *); do
    echo "$B --autocomplete bash >/etc/bash_completion.d/dfir-dd_$B" >>$DIR/maintainer/postinst
    echo "$B --autocomplete zsh >/usr/local/share/zsh/site-functions/dfir-dd_$B" >>$DIR/maintainer/postinst
done