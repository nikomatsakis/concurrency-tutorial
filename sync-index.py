#!/usr/bin/env python

import sys
import re
from urllib import urlencode

href = re.compile(r'href="src/([a-z_]+).rs"')

def replace(mo):
    path = mo.group(1)
    file_contents = file("src/%s.rs" % path).read()
    query = urlencode({"version": "stable", "code": file_contents})
    return 'href="https://play.rust-lang.org/?%s"' % query
                            
    
for line in sys.stdin:
    line2 = href.sub(replace, line)
    sys.stdout.write(line2)
    
