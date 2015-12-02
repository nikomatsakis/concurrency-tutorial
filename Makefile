index.html: index-template.html sync-index.py
	python sync-index.py < index-template.html > index.html
