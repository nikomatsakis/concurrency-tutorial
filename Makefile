index.html: index-template.md sync-index.py
	markdown index-template.md | python sync-index.py > index.html
