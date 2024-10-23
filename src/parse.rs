use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use regex::Regex;

// Estructura para almacenar un token
enum Token {
    Ident(String),
    Char(u8),
    Num(String),
    PalabraReservada(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Ident(nombre) => write!(f, "ident(\"{}\")", nombre),
            Token::Char(valor) => write!(f, "char({})", *valor as char),
            Token::Num(valor) => write!(f, "num({})", valor),
            Token::PalabraReservada(valor) => write!(f, "{}", valor),
        }
    }
}

// Función para tokenizar una línea
fn tokenizar_linea(linea: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let re = Regex::new(r"tipo:\s*([^,]+)").unwrap();

    // Captura de tokens en el formato esperado
    if let Some(captura) = re.captures(linea) {
        let valor = captura.get(1).unwrap().as_str().trim();

        if valor.starts_with("Ident(") {
            let nombre_ident = &valor[6..valor.len() - 1];
            tokens.push(Token::Ident(nombre_ident.to_string()));
        } else if valor.starts_with("Char(") {
            let char_value = &valor[6..valor.len() - 1];
            tokens.push(Token::Char(char_value.chars().next().unwrap() as u8));
        } else if valor.starts_with("Digit(") {
            let digit_value = &valor[6..valor.len() - 1];
            tokens.push(Token::Num(digit_value.to_string()));
        } else {
            tokens.push(Token::PalabraReservada(valor.to_string()));
        }
    }

    tokens
}

// Función para leer un archivo y tokenizar su contenido
fn leer_archivo_y_tokenizar(archivo: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let file = File::open(archivo).unwrap();
    let reader = BufReader::new(file);

    for linea in reader.lines() {
        let linea = linea.unwrap();
        tokens.extend(tokenizar_linea(&linea));
    }

    tokens
}

// Definición de nodos del árbol
#[derive(Debug)]
struct TreeNode {
    value: String,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(value: &str) -> TreeNode {
        TreeNode {
            value: value.to_string(),
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}

// Función para construir el árbol de parsing
fn build_parse_tree(tokens: Vec<Token>) -> Result<TreeNode, String> {
    if tokens.is_empty() {
        return Err("No tokens found".to_string());
    }

    // Nodo raíz del árbol
    let mut root = TreeNode::new("Root");

    // Lógica de construcción del árbol (basada en tokens)
    for token in tokens {
        let token_str = format!("{}", token); // Convertir el token a cadena
        match token {
            Token::PalabraReservada(ref palabra) if palabra == "let" || palabra == "const" || palabra == "var" || palabra == "func" || palabra == "if" || palabra == "then" || palabra == "else" => {
                let mut node = TreeNode::new(&token_str);
                node.add_child(TreeNode::new("Statement"));
                root.add_child(node);
            }
            Token::Char(_) | Token::Num(_) => {
                let mut node = TreeNode::new(&token_str);
                node.add_child(TreeNode::new("Value"));
                root.add_child(node);
            }
            _ => {
                // Para cualquier otro token, asumimos que es un identificador
                let node = TreeNode::new(&token_str);
                root.add_child(node);
            }
        }
    }

    Ok(root)
}

// Función para escribir la estructura del árbol en un archivo
fn write_tree_output(output_path: &str, root: &TreeNode) -> Result<(), std::io::Error> {
    let mut output_file = File::create(output_path)?;
    writeln!(output_file, "{}", format_tree(root, 0))?;
    Ok(())
}

// Función auxiliar para formatear el árbol como texto
fn format_tree(node: &TreeNode, depth: usize) -> String {
    let mut tree_str = format!("{}{}\n", "  ".repeat(depth), node.value);
    for child in &node.children {
        tree_str.push_str(&format_tree(child, depth + 1));
    }
    tree_str
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: falta el nombre del archivo de entrada");
        std::process::exit(1);
    }

    let archivo = &args[1];
    let tokens = leer_archivo_y_tokenizar(archivo);

    //let salida: Box<dyn Write> = Box::new(stdout());

    if args.len() > 2 && args[2] == "-o" {
        if args.len() < 4 {
            eprintln!("Error: falta el nombre del archivo de salida");
            std::process::exit(1);
        }

        //let archivo_salida = &args[3];
        //let file = File::create(archivo_salida).unwrap();
        //salida = Box::new(file);
    }

    // Construir el árbol de parsing
    match build_parse_tree(tokens) {
        Ok(tree) => {
            // Escribir la estructura del árbol en el archivo de salida
            if let Err(e) = write_tree_output(&args[3], &tree) {
                eprintln!("Error al escribir en el archivo de salida: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error al construir el árbol de parsing: {}", e);
            std::process::exit(1);
        }
    }
}



