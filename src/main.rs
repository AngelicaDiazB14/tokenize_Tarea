
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
use std::fs::File;
use std::fmt;
use std::io::{self, BufReader, BufWriter, Write, Read};

// Deriva automáticamente las implementaciones de Debug y Clone para la estructura Token.
// Debug permite formatear el Token para depuración, y Clone permite duplicarlo.
#[derive(Debug, Clone)]

// Define una estructura llamada Token que representará un token con tres campos:
// 'tipo' para el tipo de token, 'linea' y 'columna' para su ubicación en el archivo.
struct Token {
    tipo: TokenType,  // El tipo del token (definido más adelante como una enumeración TokenType).
    linea: usize,     // La línea donde se encuentra el token (un valor entero sin signo).
    columna: usize,   // La columna donde se encuentra el token (un valor entero sin signo).
}

// Deriva automáticamente las implementaciones de Debug y Clone para la enumeración TokenType.
// Esta enumeración definirá los posibles tipos de tokens que se pueden encontrar.
#[derive(Debug, Clone)]

// Define una enumeración (enum) llamada TokenType, que contiene varios tipos de tokens.
// Cada variante de la enumeración puede almacenar diferentes tipos de datos (números, caracteres, etc.).
enum TokenType {
    Digit(i64),        // Representa un token numérico (almacena un número entero de 64 bits).
    Char(char),        // Representa un token de carácter (almacena un carácter).
    Ident(String),     // Representa un identificador (almacena una cadena de texto).
    Op(String),        // Representa un operador (almacena una cadena de texto para el operador).
    Other(String),     // Representa otros tipos de tokens (almacena una cadena para otros valores).
}

// Implementa el trait fmt::Display para TokenType, lo que permite convertir el token
// en una cadena formateada cuando se imprime o muestra como texto.
impl fmt::Display for TokenType {
    // Define la función 'fmt', que se llama cuando queremos mostrar el valor de TokenType.
    // La función formatea el valor y lo escribe en 'f', que es un formateador.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Usa 'match' para decidir cómo formatear cada variante de TokenType.
        match self {
            // Si el TokenType es Digit, formatea la salida como "Digit(n)", donde 'n' es el número.
            TokenType::Digit(n) => write!(f, "Digit({})", n),
            // Si el TokenType es Char, formatea la salida como "Char(c)", donde 'c' es el carácter.
            TokenType::Char(c) => write!(f, "Char({})", c),
            // Si el TokenType es Ident, formatea la salida como "Ident(s)", donde 's' es la cadena.
            TokenType::Ident(ref s) => write!(f, "Ident({})", s),
            // Si el TokenType es Op, formatea la salida como "Op(s)", donde 's' es el operador.
            TokenType::Op(ref s) => write!(f, "Op({})", s),
            // Si el TokenType es Other, formatea la salida como "Other(s)", donde 's' es el valor.
            TokenType::Other(ref s) => write!(f, "Other({})", s),
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
        writeln!(writer, "{}:{} {:?}", token.linea, token.columna, token).map_err(|e| {
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
fn identifier(chars: &mut impl Iterator<Item = char>, linea: usize, columna: usize) -> Option<Token> {
    // Crea una nueva cadena para acumular los caracteres que forman el identificador.
    let mut ident_str = String::new();
    
    // Intenta obtener el siguiente carácter del iterador.
    if let Some(c) = chars.next() {
        // Verifica si el carácter es una letra.
        if letter(c) {
            // Si es una letra, añádelo a la cadena del identificador.
            ident_str.push(c);
        }
    }

    // Continúa extrayendo caracteres mientras sean letras o dígitos.
    while let Some(c) = chars.next() {
        // Verifica si el carácter es una letra o un dígito.
        if letter(c) || digit(c) {
            // Si es válido, añádelo a la cadena del identificador.
            ident_str.push(c);
        } else {
            // Si se encuentra un carácter que no es letra ni dígito, termina el bucle.
            break;
        }
    }
    
    // Crea y devuelve un Token de tipo Ident con la cadena de identificador acumulada,
    // junto con la línea y la columna en la que se encontraba el primer carácter del identificador.
    Some(Token {
        tipo: TokenType::Ident(ident_str),
        linea,
        columna,
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
        // Si el carácter es un dígito, lo añade a la cadena `num_str`.
        if digit(c) {
            num_str.push(c);
        }
    }
    
    // Mientras haya más caracteres, sigue añadiendo dígitos a `num_str`.
    while let Some(c) = chars.next() {
        // Si el carácter es un dígito, lo añade a la cadena `num_str`.
        if digit(c) {
            num_str.push(c);
        } else {
            // Si el carácter no es un dígito, se detiene el proceso de adición de caracteres.
            break;
        }
    }
    
    // Intenta convertir la cadena `num_str` a un número entero de 64 bits (i64).
    // Si la conversión es exitosa, crea un token del tipo `Digit` con el número y la posición actual.
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
fn get_spaces(chars: &mut impl Iterator<Item = char>) {
    while let Some(c) = chars.next() {
        if !space(c) {
            break;
        }
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
    matches!(c, '+' | '-' | '*' | '/' | '=' | '<' | '>' | '\\' | '&' | '@' | '%' | '^' | '?')
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
fn character(chars: &mut impl Iterator<Item = char>, linea: usize, columna: usize) -> Option<Token> {
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
//                                          other
// ===============================================================================================
// Función que reconoce otros caracteres específicos como paréntesis, puntos, comas, etc.
// Devuelve un token correspondiente al carácter reconocido.
fn other(chars: &mut impl Iterator<Item = char>, linea: usize, columna: usize) -> Option<Token> {
    if let Some(c) = chars.next() {
        match c {
            '.' => Some(Token { tipo: TokenType::Other("dot".to_string()), linea, columna }),
            ',' => Some(Token { tipo: TokenType::Other("comma".to_string()), linea, columna }),
            ';' => Some(Token { tipo: TokenType::Other("semicolon".to_string()), linea, columna }),
            '(' => Some(Token { tipo: TokenType::Other("lparen".to_string()), linea, columna }),
            ')' => Some(Token { tipo: TokenType::Other("rparen".to_string()), linea, columna }),
            '[' => Some(Token { tipo: TokenType::Other("lbracket".to_string()), linea, columna }),
            ']' => Some(Token { tipo: TokenType::Other("rbracket".to_string()), linea, columna }),
            '{' => Some(Token { tipo: TokenType::Other("lcurly".to_string()), linea, columna }),
            '}' => Some(Token { tipo: TokenType::Other("rcurly".to_string()), linea, columna }),
            _ => None,
        }
    } else {
        None
    }
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
//                                          tokenize
// ===============================================================================================
// Esta función toma un iterador de caracteres con capacidad de "peek" y convierte el contenido en una
// lista de tokens. Analiza caracteres para identificar espacios, comentarios, identificadores, números,
// operadores, caracteres y otros símbolos. Devuelve un vector de tokens generados.
fn tokenize(mut chars: std::iter::Peekable<impl Iterator<Item = char>>) -> Vec<Token> {
    // Crea un vector vacío para almacenar los tokens generados.
    let mut tokens = Vec::new();
    
    // Inicializa el número de línea y columna en 1.
    let mut linea = 1;
    let mut columna = 1;

    // Itera sobre los caracteres usando "peek" para ver el siguiente carácter sin consumirlo.
    while let Some(&c) = chars.peek() {
        // Si el carácter es un espacio (espacios en blanco, tabulaciones o saltos de línea).
        if space(c) {
            // Si el carácter es un salto de línea, incrementa el número de línea y reinicia la columna.
            if c == '\n' {
                linea += 1;
                columna = 1;
            } else {
                // Si el carácter es un espacio en blanco, simplemente incrementa la columna.
                columna += 1;
            }
            // Llama a `get_spaces` para consumir todos los espacios en blanco consecutivos.
            get_spaces(&mut chars);
        // Si el carácter es un signo de exclamación, se trata de un comentario.
        } else if c == '!' {
            // Llama a `comment` para consumir el comentario.
            comment(&mut chars);
        // Si el carácter es una letra, se trata de un identificador.
        } else if letter(c) {
            // Llama a `identifier` para extraer el identificador y, si se encuentra uno, añádelo al vector de tokens.
            if let Some(token) = identifier(&mut chars, linea, columna) {
                // Actualiza la columna después de añadir el token.
                columna += token.tipo.to_string().len();
                // Añade el token al vector de tokens.
                tokens.push(token);
            }
        // Si el carácter es un dígito, se trata de un número.
        } else if digit(c) {
            // Llama a `number` para extraer el número y, si se encuentra uno, añádelo al vector de tokens.
            if let Some(token) = number(&mut chars, linea, columna) {
                // Actualiza la columna después de añadir el token.
                columna += token.tipo.to_string().len();
                // Añade el token al vector de tokens.
                tokens.push(token);
            }
        // Si el carácter es un operador.
        } else if op_character(c) {
            // Llama a `operator` para extraer el operador y, si se encuentra uno, añádelo al vector de tokens.
            if let Some(token) = operator(&mut chars, linea, columna) {
                // Actualiza la columna después de añadir el token.
                columna += token.tipo.to_string().len();
                // Añade el token al vector de tokens.
                tokens.push(token);
            }
        // Si el carácter es un apóstrofe, se trata de un carácter.
        } else if c == '\'' {
            // Llama a `character` para extraer el carácter y, si se encuentra uno, añádelo al vector de tokens.
            if let Some(token) = character(&mut chars, linea, columna) {
                // Actualiza la columna después de añadir el token.
                columna += token.tipo.to_string().len();
                // Añade el token al vector de tokens.
                tokens.push(token);
            }
        } else {
            // Para cualquier otro carácter, intenta extraer un token de tipo "otro".
            if let Some(token) = other(&mut chars, linea, columna) {
                // Actualiza la columna después de añadir el token.
                columna += token.tipo.to_string().len();
                // Añade el token al vector de tokens.
                tokens.push(token);
            }
        }
    }
    // Devuelve el vector de tokens generados.
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
    let tokens = tokenize(buffer.chars().peekable());

    // Llamada a la función para escribir los tokens en el archivo de salida
    escribir_archivo(&output_file, tokens)?;

    Ok(())
}
