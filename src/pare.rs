use petgraph::dot::{Dot, Config};
use petgraph::graph::{Graph, NodeIndex};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::process;

// Función para leer la estructura del árbol desde un archivo
fn read_tree_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}

// Función para construir el árbol en formato DOT
fn build_tree_graph(tree_lines: &[String]) -> Graph<String, ()> {
    let mut graph = Graph::<String, ()>::new();

    // Nodo raíz
    let root = graph.add_node("Root".to_string());

    let mut current_parent = root;
    let mut stack: Vec<NodeIndex> = vec![root];

    // Construir el árbol
    for line in tree_lines {
        let indent_level = line.chars().take_while(|&c| c == ' ').count();
        let content = line.trim().to_string();

        // Si la indentación es mayor, es un hijo del nodo anterior
        if indent_level > stack.len() {
            stack.push(current_parent);
        }

        // Si la indentación es menor, retrocedemos en la pila
        while indent_level < stack.len() - 1 {
            stack.pop();
        }

        // Añadir el nodo
        let new_node = graph.add_node(content.clone());
        graph.add_edge(stack[stack.len() - 1], new_node, ());

        current_parent = new_node;
    }

    graph
}

// función para escribir el archivo en formato dot
fn write_dot_file(graph: &Graph<String, ()>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let dot_representation = format!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]));
    file.write_all(dot_representation.as_bytes())?;
    Ok(())
}

fn main() {
    // Lee los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    // archivo predeterminado si no se especifica uno
    let mut file_path = "tree.out";
    let output_path = "tree.dot"; // Archivo DOT de salida

    // si se proporciona un archivo en la línea de comandos, usarlo
    if args.len() > 1 {
        file_path = &args[1];
    }

    // lee el archivo con la estructura del árbol
    match read_tree_from_file(file_path) {
        Ok(tree_lines) => {
            println!("Generando visualización del árbol de parsing...");

            // construir el gráfico del árbol
            let graph = build_tree_graph(&tree_lines);

            // escribir el gráfico en formato DOT
            match write_dot_file(&graph, output_path) {
                Ok(_) => println!("Archivo DOT generado exitosamente: {}", output_path),
                Err(e) => {
                    eprintln!("Error al escribir el archivo DOT: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error al leer el archivo de árbol: {}", e);
            process::exit(1);
        }
    }
}
