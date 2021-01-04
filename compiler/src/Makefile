PROGRAM=test

LEX_FILE=lex.yy.c
LEX_SRC=flex.l
YACC_FILE=bison.tab.c
YACC_SRC=bison.y

LEX=flex
YACC=bison



all: $(PROGRAM)

$(PROGRAM): $(LEX_FILE) $(YACC_FILE)
	cc -o $(PROGRAM) $(LEX_FILE) $(YACC_FILE) -lfl

$(LEX_FILE): $(LEX_SRC)
	$(LEX) $(LEX_SRC)

$(YACC_FILE): $(YACC_SRC)
	$(YACC) -d $(YACC_SRC)

clean:
	rm *~
distclean:
	rm *.c *.h
