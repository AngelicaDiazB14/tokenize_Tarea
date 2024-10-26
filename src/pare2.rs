// Importa las estructuras Graph y NodeIndex de la librería petgraph, necesarias para la creación del grafo y el índice de nodos.
use petgraph::graph::{Graph, NodeIndex};
// Importa el módulo fs de la biblioteca estándar para el manejo de archivos.
use std::fs;
// Importa la función Read para leer contenido de archivos.
use std::io::Read;
// Importa VecDeque, una estructura de datos que permite manejar el grafo como una cola de nodos.
use std::collections::VecDeque;
// Define la estructura ASTNode, que representa un nodo en el árbol sintáctico abstracto.
#[derive(Debug)]
struct ASTNode {
    label: String,  // Campo para almacenar la etiqueta del nodo.
}

// Implementa métodos para la estructura ASTNode.
impl ASTNode {
    // Método constructor para crear un nuevo ASTNode dado un label (etiqueta).
    fn new(label: &str) -> Self {
        ASTNode {
            label: label.to_string(),  // Convierte el label en una String.
        }
    }
}


// =================================================================================================
//                                           parse_tree
// =================================================================================================

// Función que analiza una representación en texto de un árbol y la convierte en un grafo.
fn parse_tree(content: &str) -> Graph<ASTNode, ()> {
    let mut graph = Graph::<ASTNode, ()>::new();  // Crea un nuevo grafo vacío.
    let mut stack: VecDeque<NodeIndex> = VecDeque::new();  // Pila para almacenar los índices de nodos.
    let mut current_label = String::new();  // Variable para construir etiquetas de nodos.

    // Itera sobre cada línea del contenido.
    for line in content.lines() {
        let line = line.trim();  // Elimina espacios en blanco al inicio y al final.

        // Itera sobre cada carácter en la línea actual.
        for token in line.chars() {
            match token {
                '(' => {  // Si encuentra un '(', crea un nuevo nodo.
                    if !current_label.is_empty() {  // Si la etiqueta no está vacía.
                        let parent = stack.back().copied();  // Obtiene el nodo padre actual.
                        let node_index = graph.add_node(ASTNode::new(&current_label));  // Añade el nodo al grafo.
                        if let Some(parent_index) = parent {
                            graph.add_edge(parent_index, node_index, ());  // Conecta el nodo al padre.
                        }
                        stack.push_back(node_index);  // Añade el nodo actual a la pila.
                        current_label.clear();  // Limpia la etiqueta actual.
                    }
                }
                ')' => {  // Si encuentra un ')', cierra un nodo.
                    if !current_label.is_empty() {
                        let parent = stack.back().copied();
                        let node_index = graph.add_node(ASTNode::new(&current_label));
                        if let Some(parent_index) = parent {
                            graph.add_edge(parent_index, node_index, ());
                        }
                        current_label.clear();
                    }
                    stack.pop_back();  // Saca el nodo actual de la pila.
                }
                ',' => {  // Si encuentra una ',', indica el fin de un nodo hermano.
                    if !current_label.is_empty() {
                        let parent = stack.back().copied();
                        let node_index = graph.add_node(ASTNode::new(&current_label));
                        if let Some(parent_index) = parent {
                            graph.add_edge(parent_index, node_index, ());
                        }
                        current_label.clear();
                    }
                }
                _ => {  
                    current_label.push(token);
                }
            }
        }
    }

    graph  // Retorna el grafo resultante.
}


// =================================================================================================
//                                          sanitize_label
// =================================================================================================
// Función que reemplaza las comillas dobles para que no genere problemas en el .dot
fn sanitize_label(label: &str) -> String {
    // Reemplaza comillas dobles en la etiqueta con comillas simples.
    label.replace('"', "'")
}


// =================================================================================================
//                                         print_graph_dot
// =================================================================================================
// Función para imprimir el grafo en formato DOT.
fn print_graph_dot(graph: &Graph<ASTNode, ()>) {
    println!("digraph G {{");  // Inicia el bloque DOT.
    for node_index in graph.node_indices() {
        let node = &graph[node_index];  // Obtiene el nodo en el índice actual.
        let sanitized_label = sanitize_label(&node.label);  // Sanitiza la etiqueta.
        println!("    {} [label=\"{}\"];", node_index.index(), sanitized_label);  // Imprime el nodo en formato DOT.
    }
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();  // Obtiene los nodos de origen y destino.
        println!("    {} -> {};", source.index(), target.index());  // Imprime la arista en formato DOT.
    }
    println!("}}");  // Cierra el bloque DOT.
}


// =================================================================================================
//                                             main
// =================================================================================================
fn main() {
    let args: Vec<String> = std::env::args().collect();  // Lee los argumentos de línea de comandos.
    let filename = if args.len() > 1 { &args[1] } else { "tree.out" };  // Determina el nombre del archivo, si no se ingresa en la línea de comandos

    let mut file = fs::File::open(filename).expect("Error al abrir el archivo");  // Abre el archivo de entrada.
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error al leer el archivo");  // Lee el contenido del archivo.

    let graph = parse_tree(&content);  // Genera el grafo a partir del contenido del archivo.

    // Imprime el grafo en formato DOT.
    print_graph_dot(&graph);
}
