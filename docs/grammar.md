Program <- (ConstStatement / Fn / StructDefinition)*
ConstStatement <- 'const' Id '=' ConstExpr

Fn <- 'fn' Id '(' Arg* ')' ('->' Type)? '{' Statement* '}'
Arg <- Id ':' Type

Statement <- VarDeclaration / Assignment / ForLoop / WhileLoop / (Expr ';')

VarDeclaration <- 'let' 'mut'? Id '=' Expr ';'
Assignment <- Id '=' Expr ';'
Mutate <- Id ('+=' / '-=' '/=' / '*=' / '^=' / '&=' / '|=' / '&&=' / '||=') Expr

ForLoop <- 'for' Id 'in' Expr '..' Expr '{' Statement* '}'
WhileLoop <- 'while' Expr '{' Statement* '}'

Expr <- Term (('&&' / '||') Term)*
Term <- ArithExpr (('==' / '!=') ArithExpr)*

ArithExpr <- ArithTerm (('+' / '-') ArithTerm)* 
ArithTerm <- BitExpr (('*' / '/') BitExpr)*
BitExpr <- Factor (('&' / '|' / '^') Factor)*

Factor <-  Num / Id / '(' Expr ')' 

Num <- [0-9]+
Id <- [a-zA-Z_]+

Type <- 'u8' / 'u16' / 'u32' / 'u64' / 'i8' / 'i16' / 'i32' / 'i64' / ('struct' Id) / (Type '[' Num ']') / ('&' Type)
