%{
#include<stdio.h>
#include<stdlib.h>
#include<string.h>
    %}

%union{
    int num;
    char * str;
 }

%token <num> NUM
%token <str> ID
%token ADD SUB MUL DIV MOD
%token SEMI LP LB RP RB
%token FOR MAIN IF ELSE
%token INT LED
%token ASSIGN ADDASSIGN DEVASSIGN MULASSIGN SUBASSIGN
%token EQ BT BTEQ LT LTEQ

%%
CODE:           MAIN LP RP BLOCK;

BLOCK:          LB BODY RB;

BODY:           DECLPART STATEMENTS
|       STATEMENTS
|       DECLPART
;

DECLPART:       DECLPART DECL
|DECL
;

DECL:           INT IDs SEMI;

IDs:            IDs ID
|ID;

STATEMENTS:     STATEMENTS ST
|ST;

ST:             exp SEMI
|COMP SEMI
|DISP SEMI


;

DISP: LED LP exp RP
| LED LP COMP RP
| LED LP ID  RP
;

exp: factor
| exp ADD factor
| exp SUB factor
;

factor: term
| factor MUL term 
| factor DIV term
;

term: NUM
|LP exp RP
;

COMP: exp EQ exp
|exp BT exp
|exp LT exp
|exp BTEQ exp
|exp LTEQ exp
;

ASSIGNS: ID ASSIGN exp
|ID ADDASSIGN exp
|ID MULASSIGN exp
|ID SUBASSIGN exp
|ID DIVASSIGN exp

%%

main(int argc, char **argv){
    yyparse();
}

yyerror(char *s){
    fprintf(stderr, "error: %s\n",s);
}
