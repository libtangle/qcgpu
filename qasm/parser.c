// The following is an implementation of OpenQASM
// using a recursive descent style parser.

// MainProgram := "OPENQASM" Real ";" Program
// Program ::= Statement | Program Statement
// Statement ::= Decl 
//             | GateDecl GeoList "}" 
//             | GateDecl "}"
//             | "opaque" Id Idlist ";"
//             | "opaque" Id "()" IdList ";"
//             | "opaque" Id "(" IdList ")" IdList ";"
//             | Qop
//             | "if" "(" Id "==" Integer ")" Qop
//             | "barrier" AnyList ";"
// Decl ::= "qreg" Id "[" Integer "]" ";" | "creg" Id "[" Integer "]" ";"
// GateDecl ::= "gate" Id IdList "{" | "gate" Id "()" IdList "{" | "gate" Id "(" IdList ")" Idlist "{"
// GopList ::= Uop | "barrier" IdList ";" | GopList Uop | GopList "barrier" IdList ";"
// Qop ::= Uop | "measure" Argument "->" Argument ";" | "reset" Argument ";"
// Uop ::= "U" "(" ExpList ")" Argument ";" | "CX" Argument "," Argument ";" | Id AnyList ";" | Id "()" AnyList ";" | Id ""
