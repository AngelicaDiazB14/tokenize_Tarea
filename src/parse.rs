
//================================================================================================
//                                          parse
//================================================================================================

use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};

use serde::{Serialize, Deserialize};
use serde_json;



struct ParseError {
    message: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError { message: msg.to_string() }
    }
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
fn build_parse_tree(tokens: Vec<String>) -> Result<TreeNode, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::new("No tokens found"));
    }

    // Nodo raíz del árbol
    let mut root = TreeNode::new("Root");

    // Lógica de construcción del árbol (basada en tokens)
    for token in tokens {
        match token.as_str() {
            "let" | "const" | "var" | "func" | "in" | "if" | "then" | "else" => {
                let mut node = TreeNode::new(&token);
                node.add_child(TreeNode::new("Statement"));
                root.add_child(node);
            }
            "ord('a')" | "ord('A')" => {
                let mut node = TreeNode::new(&token);
                node.add_child(TreeNode::new("FunctionCall"));
                root.add_child(node);
            }
            "~" | "-" | ":" | ";" | "!" | "(" | ")" => {
                let mut node = TreeNode::new(&token);
                node.add_child(TreeNode::new("Operator"));
                root.add_child(node);
            }
            _ => {
                // Para cualquier token que no sea una palabra clave o símbolo, asumimos que es un valor
                let node = TreeNode::new(&token);
                root.add_child(node);
            }
        }
    }

    Ok(root)
}

// leer los tokens desde el archivo especificado por argumento
fn parse_tokens(file_path: &str) -> Result<Vec<String>, ParseError> {
    let file = File::open(file_path).map_err(|_| ParseError::new("Error al abrir el archivo de tokens"))?;
    let _reader = BufReader::new(file);

    //revisar, deberia ser los valores de los token que sacamos de main
    let tokens = vec![
        "let".to_string(),
        "const".to_string(),
        "shift".to_string(),
        "~".to_string(),
        "ord('a')".to_string(),
        "-".to_string(),
        "ord('A')".to_string(),
        ";".to_string(),
        "var".to_string(),
        "i".to_string(),
        ":".to_string(),
        "integer".to_string(),
        ";".to_string(),
        "func".to_string(),
        "capital".to_string(),
        "(".to_string(),
        "var".to_string(),
        "chr".to_string(),
        ":".to_string(),
        "Char".to_string(),
        ")".to_string(),
        ":".to_string(),
        "Boolean".to_string(),
        "~".to_string(),
        "(ord('A') <= ord(ch))".to_string(),
        "/\\".to_string(),
        "(ord(ch) <= ord('Z'))".to_string(),
        "in".to_string(),
        "15".to_string(),
        "!".to_string(),
        "hola".to_string(),
        "if".to_string(),
        "capital(current)".to_string(),
        "then".to_string(),
        "chr(ord(current) + shift)".to_string(),
        "else".to_string(),
        "current".to_string(),
    ];

    Ok(tokens)
}

// Función para escribir la estructura del árbol en un archivo
fn write_tree_output(output_path: &str, root: &TreeNode) -> Result<(), io::Error> {
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
/*
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut output_file = "tree.out";  // Archivo de salida predeterminado

    if args.len() < 2 {
        eprintln!("Error: No input file provided.");
        std::process::exit(1);
    }

    let input_file = &args[1];

    // Verifica si se ha proporcionado un archivo de salida con la directiva -o
    if args.len() > 3 && args[2] == "-o" {
        output_file = &args[3];
    }

    // Intenta parsear los tokens del archivo de entrada
    match parse_tokens(input_file) {
        Ok(tokens) => {
            // Construye el árbol de parsing
            match build_parse_tree(tokens) {
                Ok(tree) => {
                    // Escribe la estructura del árbol en el archivo de salida
                    if let Err(e) = write_tree_output(output_file, &tree) {
                        eprintln!("Error al escribir en el archivo de salida: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error al construir el árbol de parsing: {}", e.message);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error al leer tokens: {}", e.message);
            std::process::exit(1);
        }
    }
}
*/








// -------------------------------------- TOKEN -------------------------------------- //


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Token {
    tipo: String,
    children: Vec<Token>,
}

impl Token {
    // Constructor for creating a new Token
    fn new(tipo: String, children: Vec<Token>) -> Self {
        Token { tipo, children }
    }

    // Method to add a child token to the list of children
    fn add_child(&mut self, child: Token) {
        self.children.push(child);
    }

    // Method to convert the token to JSON format
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    // Recursive method to generate DOT representation
    fn to_dot(&self, parent: Option<&str>, dot_output: &mut String) {
        let node_name = format!("\"{}\"", self.tipo);
        if let Some(parent_name) = parent {
            dot_output.push_str(&format!("  \"{}\" -> {};\n", parent_name, node_name));
        }
        for child in &self.children {
            child.to_dot(Some(&self.tipo), dot_output);
        }
    }

}


// -------------------------------------- TOKEN -------------------------------------- //


fn main() {
    // Create a more complex token structure
    let leaf_token_1 = Token::new(String::from("Leaf 1"), vec![]);
    let leaf_token_2 = Token::new(String::from("Leaf 2"), vec![]);
    let leaf_token_3 = Token::new(String::from("Leaf 3"), vec![]);
    let leaf_token_4 = Token::new(String::from("Leaf 4"), vec![]);
    let leaf_token_5 = Token::new(String::from("Leaf 5"), vec![]);

    // Create intermediate tokens with children
    let mut mid_token_1 = Token::new(String::from("Mid 1"), vec![]);
    mid_token_1.add_child(leaf_token_1);
    mid_token_1.add_child(leaf_token_2);

    let mut mid_token_2 = Token::new(String::from("Mid 2"), vec![]);
    mid_token_2.add_child(leaf_token_3);

    let mut mid_token_3 = Token::new(String::from("Mid 3"), vec![]);
    mid_token_3.add_child(leaf_token_4);
    mid_token_3.add_child(leaf_token_5);

    // Create the root token with multiple children
    let mut root_token = Token::new(String::from("Root"), vec![]);
    root_token.add_child(mid_token_1);
    root_token.add_child(mid_token_2);
    root_token.add_child(mid_token_3);


    // Convert the token to JSON format
    let json_representation = root_token.to_json();
    println!("JSON format:\n{}", json_representation);


    // Generate the DOT representation
    let mut dot_output = String::from("digraph TokenTree {\n");
    root_token.to_dot(None, &mut dot_output);
    dot_output.push_str("}\n");

    // Print the DOT representation
    println!("DOT format:\n{}", dot_output);
}