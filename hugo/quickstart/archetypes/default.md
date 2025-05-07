+++
date = '{{ .Date }}'
draft = true
title = '{{ replace .File.ContentBaseName "-" " " | title }}'
+++


You can create an archetype for one or more content types.
For example, use one archetype for posts, and use the default archetype for everything else:
