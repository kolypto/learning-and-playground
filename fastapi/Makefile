SHELL := /bin/bash

generate_mds = a_intro.md b_di.md c_authentication.md d_db.md e_advanced.md f_graphql.md g_websockets.md h_pydantic.md tests.md
existing_mds = z_pytest.md

all: README.md

.PRECIOUS: README.md
README.md: $(generate_mds) $(existing_mds) Makefile README-header.md
	# Collect
	cat README-header.md $(generate_mds) $(existing_mds) > README.md

.INTERMEDIATE: $(generate_mds)
$(generate_mds): %.md: %.py
	echo "# $<" > $@
	echo '```python' >> $@
	cat $< >> $@
	echo '```' >> $@
	echo >> $@
