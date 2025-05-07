#!/bin/sh

izip="./sample.d/input.zip"

genzip() {
	echo generating zip file...

	mkdir -p ./sample.d

	echo hw1 >./sample.d/hw1.txt
	echo hw2 >./sample.d/hw2.txt

	find \
		./sample.d \
		-type f \
		-name '*.txt' |
		zip \
			-0 \
			-@ \
			-T \
			-v \
			-o \
			"${izip}"
}

test -f "${izip}" || genzip

fmt=yaml

which wazero | fgrep -q wazero || exec sh -c 'echo wazero missing.; exit 1'
which dasel | fgrep -q dasel || exec sh -c 'echo dasel missing.; exit 1'

cat "${izip}" |
	wazero \
		run \
		./rs-zip2meta.wasm |
	dasel \
		--read=json \
		--write=$fmt \
		--colour
