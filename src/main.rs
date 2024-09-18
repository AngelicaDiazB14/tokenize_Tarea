
// ...............................................................................................
// Tecnológico de Costa Rica                                                                     .
// Compiladores e Intérpretes, GR 40                                                             .
// Tarea corta 1: parte 1                                                                        .
//                                                                                               .
// Este programa recibe en la linea de comandos un archivo de texto y escribe en un archivo      .
// de salida (tokens.out) la secuencia de tokens leído del archivo de texto original e incluye la.
// línea y columna del archivo fuente en que fue encontrado.                                     .
// ...............................................................................................

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

// Deriva automáticamente las implementaciones de Debug y Clone para la estructura Token.
// Debug permite formatear el Token para depuración, y Clone permite duplicarlo.
#[derive(Debug, Clone)]

// Define una estructura llamada Token que representará un token con tres campos:
// 'tipo' para el tipo de token, 'linea' y 'columna' para su ubicación en el archivo.
struct Token {
    tipo: TokenType, // El tipo del token (definido más adelante como una enumeración TokenType).
    linea: usize,    // La línea donde se encuentra el token (un valor entero sin signo).
    columna: usize,  // La columna donde se encuentra el token (un valor entero sin signo).
}

// Deriva automáticamente las implementaciones de Debug y Clone para la enumeración TokenType.
// Esta enumeración definirá los posibles tipos de tokens que se pueden encontrar.
#[derive(Debug, Clone)]

// Define una enumeración (enum) llamada TokenType, que contiene varios tipos de tokens.
// Cada variante de la enumeración puede almacenar diferentes tipos de datos (números, caracteres, etc.).
enum TokenType {
    Digit(i64),
    Char(char),
    Ident(String),
    Op(String),
    Slash,         // /
    Assign,        // :=
    Equal,         // =
    Tilde,         // ~
    LParen,
    RParen,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Array,     // Nueva variante para "array"
    Begin,     // Nueva variante para "begin"
    Const,     // Nueva variante para "const"
    Do,        // Nueva variante para "do"
    Else,      // Nueva variante para "else"
    End,       // Nueva variante para "end"
    Func,      // Nueva variante para "func"
    If,        // Nueva variante para "if"
    In,        // Nueva variante para "in"
    Let,       // Nueva variante para "let"
    Of,        // Nueva variante para "of"
    Proc,      // Nueva variante para "proc"
    Record,    // Nueva variante para "record"
    Then,      // Nueva variante para "then"
    Type,      // Nueva variante para "type"
    Var,       // Nueva variante para "var"
    While,     // Nueva variante para "while"
}

// Palabras reservadas
fn es_palabra_reservada(s: &str) -> Option<TokenType> {
    match s {
        "array" => Some(TokenType::Array),
        "begin" => Some(TokenType::Begin),
        "const" => Some(TokenType::Const),
        "do" => Some(TokenType::Do),
        "else" => Some(TokenType::Else),
        "end" => Some(TokenType::End),
        "func" => Some(TokenType::Func),
        "if" => Some(TokenType::If),
        "in" => Some(TokenType::In),
        "let" => Some(TokenType::Let),
        "of" => Some(TokenType::Of),
        "proc" => Some(TokenType::Proc),
        "record" => Some(TokenType::Record),
        "then" => Some(TokenType::Then),
        "type" => Some(TokenType::Type),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None, // Si no es una palabra reservada, devuelve None
    }

// Implementa el trait fmt::Display para TokenType, lo que permite convertir el token
// en una cadena formateada cuando se imprime o muestra como texto.
// Implementa el trait fmt::Display para TokenType, lo que permite convertir el token
// en una cadena formateada cuando se imprime o muestra como texto.
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Digit(n) => write!(f, "Digit({})", n),
            TokenType::Char(c) => write!(f, "Char('{}')", c),
            TokenType::Ident(ref s) => write!(f, "Ident({})", s),
            TokenType::Op(ref s) => write!(f, "Op('{}')", s),
            TokenType::Slash => write!(f, "/"),
            TokenType::Assign => write!(f, ":="),
            TokenType::Equal => write!(f, "="),
            TokenType::Tilde => write!(f, "~"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBracket => write!(f, "["),
            TokenType::RBracket => write!(f, "]"),
            TokenType::LCurly => write!(f, "{{"),
            TokenType::RCurly => write!(f, "}}"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Colon => write!(f, ":"),
            TokenType::Array => write!(f, "array"),
            TokenType::Begin => write!(f, "begin"),
            TokenType::Const => write!(f, "const"),
            TokenType::Do => write!(f, "do"),
            TokenType::Else => write!(f, "else"),
            TokenType::End => write!(f, "end"),
            TokenType::Func => write!(f, "func"),
            TokenType::If => write!(f, "if"),
            TokenType::In => write!(f, "in"),
            TokenType::Let => write!(f, "let"),
            TokenType::Of => write!(f, "of"),
            TokenType::Proc => write!(f, "proc"),
            TokenType::Record => write!(f, "record"),
            TokenType::Then => write!(f, "then"),
            TokenType::Type => write!(f, "type"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),
        }
    }
}

// ===============================================================================================
//                                          leer_archivo
// ===============================================================================================
// Función que abre un archivo de entrada y lee su contenido en una cadena de texto.
// Recibe el nombre del archivo como parámetro y devuelve un 'Result' con el contenido leído.
fn leer_archivo(archivo_entrada: &str) -> io::Result<String> {
    let file = File::open(archivo_entrada).map_err(|e| {
        eprintln!("Error al abrir el archivo de entrada: {}", e);
        e
    })?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).map_err(|e| {
        eprintln!("Error al leer el archivo de entrada: {}", e);
        e
    })?;
    Ok(buffer)
}

// ===============================================================================================
//                                          escribir_archivo
// ===============================================================================================
// Función que toma una lista de tokens y escribe cada uno en un archivo de salida.
// Recibe el nombre del archivo de salida y la lista de tokens como parámetros.
fn escribir_archivo(archivo_salida: &str, tokens: Vec<Token>) -> io::Result<()> {
    let output = File::create(archivo_salida).map_err(|e| {
        eprintln!("Error al crear el archivo de salida: {}", e);
        e
    })?;
    let mut writer = BufWriter::new(output);

    for token in tokens {
        writeln!(
            writer,
            "{}:{} Token {{ tipo: {}, linea: {}, columna: {} }}",
            token.linea,
            token.columna,
            token.tipo,
            token.linea,
            token.columna
        ).map_err(|e| {
            eprintln!("Error al escribir en el archivo de salida: {}", e);
            e
        })?;
    }

    Ok(())
}

// ===============================================================================================
//                                          identifier
// ===============================================================================================
// Esta función intenta extraer un identificador del flujo de caracteres. Un identificador es una
// secuencia de caracteres alfanuméricos que comienza con una letra. Devuelve un Token de tipo Ident
// si se encuentra una secuencia válida, de lo contrario, devuelve None.

fn identifier(
    chars: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    linea: usize,
    mut columna: usize,
) -> Option<Token> {
    let mut ident_str = String::new();
    let start_columna = columna;

    // Extrae el primer carácter y verifica si es una letra
    if let Some(&c) = chars.peek() {
        if letter(c) {
            ident_str.push(c);
            chars.next(); // Consume el carácter
            columna += 1;
        } else {
            return None; // Si el primer carácter no es una letra, no es un identificador
        }
    } else {
        return None; // Si no hay ningún carácter, tampoco hay identificador
    }

    // Continua con los caracteres siguientes, verificando si son letras o dígitos
    while let Some(&c) = chars.peek() {
        if letter(c) || digit(c) {
            ident_str.push(c);
            chars.next(); // Consume el carácter
            columna += 1;
        } else {
            break; // Si se encuentra un carácter no válido, termina el bucle
        }
    }

    // Verifica si el identificador es una palabra reservada
    if let Some(reserved_type) = es_palabra_reservada(&ident_str) {
        return Some(Token {
            tipo: reserved_type,
            linea,
            columna: start_columna,
        });
    }

    // Si no es una palabra reservada, es un identificador normal
    Some(Token {
        tipo: TokenType::Ident(ident_str),
        linea,
        columna: start_columna,
    })
}


// ===============================================================================================
//                                          digit
// ===============================================================================================
// Función auxiliar que determina si un carácter es un dígito numérico. Devuelve 'true' si el
// carácter es un dígito del 0 al 9, y 'false' en caso contrario.
fn digit(c: char) -> bool {
    c.is_digit(10)
}

// ===============================================================================================
//                                          number
// ===============================================================================================
// Esta función procesa una secuencia de caracteres para identificar y extraer un número entero.
// Devuelve un token que representa el número, si se encuentra uno. El número se construye a partir
// de caracteres consecutivos que representan dígitos.
fn number(chars: &mut impl Iterator<Item = char>, linea: usize, columna: usize) -> Option<Token> {
    // Crea una cadena mutable para construir el número a partir de caracteres.
    let mut num_str = String::new();

    // Intenta obtener el siguiente carácter del iterador.
    if let Some(c) = chars.next() {
        // Si el carácter es un dígito, lo añade a la cadena num_str.
        if digit(c) {
            num_str.push(c);
        }
    }

    // Mientras haya más caracteres, sigue añadiendo dígitos a num_str.
    while let Some(c) = chars.next() {
        // Si el carácter es un dígito, lo añade a la cadena num_str.
        if digit(c) {
            num_str.push(c);
        } else {
            // Si el carácter no es un dígito, se detiene el proceso de adición de caracteres.
            break;
        }
    }

    // Intenta convertir la cadena num_str a un número entero de 64 bits (i64).
    // Si la conversión es exitosa, crea un token del tipo Digit con el número y la posición actual.
    num_str.parse::<i64>().ok().map(|n| Token {
        tipo: TokenType::Digit(n),
        linea,
        columna,
    })
}

// ===============================================================================================
//                                          operator
// ===============================================================================================
// Esta función intenta extraer un operador del flujo de caracteres. Un operador es una secuencia
// de caracteres que corresponden a símbolos de operación como '+', '-', '*', etc. Devuelve un Token
// de tipo Op si se encuentra una secuencia de operadores, de lo contrario, devuelve None.

fn operator(chars: &mut impl Iterator<Item = char>, linea: usize, columna: usize) -> Option<Token> {
    // Crea una nueva cadena para acumular los caracteres que forman el operador.
    let mut op_str = String::new();

    // Intenta obtener el siguiente carácter del iterador.
    if let Some(c) = chars.next() {
        // Verifica si el carácter es un símbolo de operador válido.
        if op_character(c) {
            // Si es un operador, añádelo a la cadena de operadores.
            op_str.push(c);
        }
    }

    // Continúa extrayendo caracteres mientras sean válidos como operadores.
    while let Some(c) = chars.next() {
        // Verifica si el carácter es un símbolo de operador válido.
        if op_character(c) {
            // Si es un operador, añádelo a la cadena de operadores.
            op_str.push(c);
        } else {
            // Si se encuentra un carácter que no es un operador, termina el bucle.
            break;
        }
    }

    // Crea y devuelve un Token de tipo Op con la cadena de operadores acumulada,
    // junto con la línea y la columna en la que se encontraba el primer carácter del operador.
    Some(Token {
        tipo: TokenType::Op(op_str),
        linea,
        columna,
    })
}

// ===============================================================================================
//                                          get_spaces
// ===============================================================================================
// Función que avanza el iterador saltando los espacios en blanco, las tabulaciones y las nuevas líneas.
// No devuelve ningún valor; simplemente mueve el cursor hasta que encuentra un carácter no vacío.
fn get_spaces(chars: &mut std::iter::Peekable<impl Iterator<Item = char>>, columna: &mut usize, linea: &mut usize) {
    while let Some(&c) = chars.peek() {
        if c == ' ' {
            *columna += 1;
        } else if c == '\t' {
            *columna += 4; // Asumiendo tabulación de 4 espacios
        } else if c == '\n' {
            *linea += 1;
            *columna = 1;
        } else {
            break;
        }
        chars.next();
    }
}


// ===============================================================================================
//                                          letter
// ===============================================================================================
// Función auxiliar que determina si un carácter es una letra del alfabeto ASCII (mayúscula o minúscula).
// Devuelve 'true' si el carácter es una letra, y 'false' en caso contrario.
fn letter(c: char) -> bool {
    c.is_ascii_alphabetic()
}

// ===============================================================================================
//                                          op_character
// ===============================================================================================
// Función auxiliar que verifica si un carácter es un operador válido según la gramática especificada.
// Devuelve 'true' si el carácter es uno de los operadores definidos, y 'false' en caso contrario.
fn op_character(c: char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '=' | '<' | '>' | '\\' | '&' | '@' | '%' | '^' | '?'
    )
}

// ===============================================================================================
//                                          space
// ===============================================================================================
// Función auxiliar que verifica si un carácter es un espacio en blanco, tabulación o nueva línea.
// Devuelve 'true' si el carácter es un espacio, y 'false' en caso contrario.
fn space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n')
}

// ===============================================================================================
//                                          character
// ===============================================================================================
// Esta función intenta extraer un carácter delimitado por comillas simples ('') del flujo de
// caracteres. Devuelve un Token de tipo Char si se encuentra un carácter válido, de lo contrario,
// devuelve None.
fn character(
    chars: &mut impl Iterator<Item = char>,
    linea: usize,
    columna: usize,
) -> Option<Token> {
    // Intenta obtener el siguiente carácter del iterador.
    if let Some(c) = chars.next() {
        // Verifica si el carácter es una comilla simple de apertura.
        if c == '\'' {
            // Intenta obtener el siguiente carácter del iterador.
            if let Some(c) = chars.next() {
                // Verifica si el siguiente carácter es una comilla simple de cierre.
                if chars.next() == Some('\'') {
                    // Si se encuentra un carácter válido y está correctamente delimitado,
                    // crea y devuelve un Token de tipo Char con el carácter extraído,
                    // así como la línea y la columna en la que se encontraba el carácter.
                    return Some(Token {
                        tipo: TokenType::Char(c),
                        linea,
                        columna,
                    });
                }
            }
        }
    }
    // Si no se encuentra un carácter válido o el delimitador no está correcto,
    // devuelve None indicando que no se pudo extraer un carácter válido.
    None
}


// ===============================================================================================
//                                          comment
// ===============================================================================================
// Función que salta los comentarios en el código fuente. Si detecta un carácter de exclamación (!),
// avanza hasta encontrar el fin de línea o el final del archivo, ignorando todo el contenido entre ellos.
fn comment(chars: &mut impl Iterator<Item = char>) {
    while let Some(c) = chars.next() {
        if c == '\n' {
            break;
        }
    }
}

// ===============================================================================================
//                                         get_token_length
// ===============================================================================================
//Devuelve la longitud del contenido del token
fn get_token_length(token: &Token) -> usize {
    match &token.tipo {
        // Números y caracteres individuales
        TokenType::Digit(n) => n.to_string().len(),         // Longitud del número en cadena
        TokenType::Char(c) => c.to_string().len(),          // Longitud del carácter en cadena
        
        // Identificadores y operadores
        TokenType::Ident(ref s) => s.len(),                 // Longitud del identificador en cadena
        TokenType::Op(ref s) => s.len(),                    // Longitud del operador en cadena

        // Símbolos de un solo carácter
        TokenType::LParen | TokenType::RParen => 1,         // Paréntesis ocupan 1 carácter
        TokenType::LBracket | TokenType::RBracket => 1,     // Corchetes ocupan 1 carácter
        TokenType::LCurly | TokenType::RCurly => 1,         // Llaves ocupan 1 carácter
        TokenType::Semicolon | TokenType::Colon => 1,       // Punto y coma y dos puntos ocupan 1 carácter
        TokenType::Comma => 1,                              // Coma ocupa 1 carácter
        TokenType::Dot => 1,                                // Punto ocupa 1 carácter
        TokenType::Slash => 1,                              // Diagonal ocupa 1 carácter
        TokenType::Equal => 1,                              // Igual "=" ocupa 1 carácter
        TokenType::Tilde => 1,                              // Tilde "~" ocupa 1 carácter

        // Operadores multicarácter
        TokenType::Assign => 2,                             // Asignación ":=" ocupa 2 caracteres

        // Palabras reservadas (cada una tiene su longitud específica)
        TokenType::Array => 5,      // "array" tiene 5 caracteres
        TokenType::Begin => 5,      // "begin" tiene 5 caracteres
        TokenType::Const => 5,      // "const" tiene 5 caracteres
        TokenType::Do => 2,         // "do" tiene 2 caracteres
        TokenType::Else => 4,       // "else" tiene 4 caracteres
        TokenType::End => 3,        // "end" tiene 3 caracteres
        TokenType::Func => 4,       // "func" tiene 4 caracteres
        TokenType::If => 2,         // "if" tiene 2 caracteres
        TokenType::In => 2,         // "in" tiene 2 caracteres
        TokenType::Let => 3,        // "let" tiene 3 caracteres
        TokenType::Of => 2,         // "of" tiene 2 caracteres
        TokenType::Proc => 4,       // "proc" tiene 4 caracteres
        TokenType::Record => 6,     // "record" tiene 6 caracteres
        TokenType::Then => 4,       // "then" tiene 4 caracteres
        TokenType::Type => 4,       // "type" tiene 4 caracteres
        TokenType::Var => 3,        // "var" tiene 3 caracteres
        TokenType::While => 5,      // "while" tiene 5 caracteres
    }
}


// ===============================================================================================
//                                          tokenize
// ===============================================================================================
// Esta función toma un iterador de caracteres con capacidad de "peek" y convierte el contenido en una
// lista de tokens. Analiza caracteres para identificar espacios, comentarios, identificadores, números,
// operadores, caracteres y otros símbolos. Devuelve un vector de tokens generados.
fn tokenize(chars: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut linea = 1;
    let mut columna = 1;

    while let Some(&c) = chars.peek() {
        if space(c) {
            get_spaces(chars, &mut columna, &mut linea);
        } else if c == '!' {
            comment(chars);
            linea += 1;
            columna = 1;  // Reinicia la columna al encontrar un nuevo salto de línea.
        } else if letter(c) {
            if let Some(token) = identifier(chars, linea, columna) {
                columna += get_token_length(&token); 
                tokens.push(token);
            }
        } else if digit(c) {
            if let Some(token) = number(chars, linea, columna) {
                columna += get_token_length(&token); 
                tokens.push(token);
            }
        } else if op_character(c) {
            if let Some(token) = operator(chars, linea, columna) {
                columna += get_token_length(&token);
                tokens.push(token);
            }
        } else if c == '\'' {
            if let Some(token) = character(chars, linea, columna) {
                columna += get_token_length(&token); 
                tokens.push(token);
            }
        } else {
            // Aquí es donde manejamos los delimitadores como paréntesis, corchetes, llaves, etc.
            match c {
                '(' => tokens.push(Token { tipo: TokenType::LParen, linea, columna }),
                ')' => tokens.push(Token { tipo: TokenType::RParen, linea, columna }),
                '[' => tokens.push(Token { tipo: TokenType::LBracket, linea, columna }),
                ']' => tokens.push(Token { tipo: TokenType::RBracket, linea, columna }),
                '{' => tokens.push(Token { tipo: TokenType::LCurly, linea, columna }),
                '}' => tokens.push(Token { tipo: TokenType::RCurly, linea, columna }),
                ',' => tokens.push(Token { tipo: TokenType::Comma, linea, columna }),
                ';' => tokens.push(Token { tipo: TokenType::Semicolon, linea, columna }),
                ':' => {
                    // Si el siguiente carácter es un '=', es un "Assign (:=)"
                    chars.next();
                    if let Some('=') = chars.peek() {
                        chars.next();
                        tokens.push(Token { tipo: TokenType::Assign, linea, columna });
                        columna += 1; // El operador := tiene dos caracteres
                    } else {
                        tokens.push(Token { tipo: TokenType::Colon, linea, columna });
                    }
                },
                '.' => tokens.push(Token { tipo: TokenType::Dot, linea, columna }),
                '/' => tokens.push(Token { tipo: TokenType::Slash, linea, columna }),
                '=' => tokens.push(Token { tipo: TokenType::Equal, linea, columna }),
                '~' => tokens.push(Token { tipo: TokenType::Tilde, linea, columna }),
                _ => {
                    // Si no es un símbolo reconocido, puedes manejar el error aquí o ignorarlo
                    // chars.next() se asegura de no quedar en un ciclo infinito
                    chars.next();
                    continue;
                }
            }
            
            // Avanza el iterador y actualiza la columna después de procesar el símbolo.
            chars.next();  // Avanza el carácter actual
            columna += 1;  // Aumenta la columna después de procesar el carácter
        }
    }

    tokens
}
// ===============================================================================================
//                                          main
// ===============================================================================================
// Función principal que maneja la lógica de entrada y salida de archivos. Recoge los argumentos,
// llama a la función que lee el archivo de entrada y escribe los tokens generados en el archivo
// de salida tokens.out o el especificado por el usuario.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut output_file = "output.tok".to_string();

    if args.len() < 2 || args.len() > 4 {
        eprintln!("Uso: {} <archivo_entrada> [-o <archivo_salida>]", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];

    if args.len() == 4 && args[2] == "-o" {
        output_file = args[3].clone();
    } else if args.len() == 3 {
        output_file = args[2].clone();
    }

    // Llamada a la función para leer el archivo
    let buffer = leer_archivo(input_file)?;

    // Tokenización
    let tokens = tokenize(&mut buffer.chars().peekable());

    // Llamada a la función para escribir los tokens en el archivo de salida
    escribir_archivo(&output_file, tokens)?;

    Ok(())
}