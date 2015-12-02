index.html: index-template.md sync-index.py $(wildcard src/*rs)
	markdown index-template.md | python sync-index.py > index.html
