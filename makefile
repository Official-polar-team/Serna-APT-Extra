DESTDIR=$ 
PREFIX=/usr/local/

install:
	chmod a+x add-source
	cp add-source $(DESTDIR)$(PREFIX)/bin

remove:
	rm -f $(DESTDIR)$(PREFIX)/bin/add-source