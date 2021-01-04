%{
int chars = 0;
int words = 0;
int lines = 0;
#include "bison.tab.h"
%}

%%
"for" {chars+=3;return FOR;}
"main" {chars+=4;return MAIN;}
"if" {chars+=2;return IF;}
"else" {chars+=4;return ELSE;}
"int" {chars+=3; return INT;}
"led" {chars+=3; return LED;}


"*" {chars++;return MUL;}
"-" {chars++;return SUB;}
"+" {chars++;return ADD;}
"/" {chars++;return DIV;}
"%" {chars++;return MOD;}

"==" {chars+=2; return EQ;}
">"  {chars++; return BT;}
">="  {chars+=2; return BTEQ;}
"<"  {chars++; return LT;}
"<="  {chars++; return LTEQ;}

"=" {chars++; return ASSIGN;}
"+=" {chars+=2; return ADDASSIGN;}
"*=" {chars+=2; return MULASSIGN;}
"-=" {chars+=2; return SUBASSIGN;}
"/=" {chars+=2; return DEVASSIGN;}

"{" {chars++;return LB;}
"}" {chars++;return RB;}
"(" {chars++;return LP;}
")" {chars++;return RP;}
";" {chars++;return SEMI;}

[a-zA-Z_][0-9a-zA-Z_]* {
char *ptr;
int len;
len = strlen(yytext);
chars += len;
ptr = malloc(sizeof(char) * len + 1);
yylval.str = ptr;
return ID;
}

[0-9]+ {
chars += strlen(yytext);
 yylval.num = atoi(yytext);
return NUM;
}

"\n" {lines++;chars=0;}
. {/*do nothing*/}
%%
