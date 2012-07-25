all:
	rustc --lib datetime.rc 

test:
	rustc --bin datetime.rc  --test && ./datetime

clean:
	rm -rf datetime *.so