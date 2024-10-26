
### Comandos para compilar el proyecto

# tokenize_Tarea -> cargo run --bin main input.tri -o output.tok
# tokenize_Tarea -> cargo run --bin parse output.tok tree.out
# tokenize_Tarea -> cargo run --bin pare tree.out



### Descripción de los archivos

- **main.rs**: tokenizador de triangle (Tarea 1)
    - se realizaron ajustes, por lo tanto, si desea ingresar un nuevo input.tri, deberá ejecutar el comando como se indica arriba. 
    - Como resultado genera los tokens en un archivo output.tok

- **parse.rs**: a partir del archivo de tokens generado en la etapa 1, se utiliza parse.rs para crear el árbol sintático de Triangle (Tarea 2)
    - Como parámetro de entrada recibe un archivo output.tok con los tokens de Triangle generado por el main.rs 
    - Como resultado se obtiene un archivo tree.out en el cual se tiene el arbol sintático de Triangle 

- **pare.rs**: toma el árbol sintático generado por parse.rs para convertirlo a formato .dot de modo que se pueda obtener una visualización gráfica del árbol (Tarea 2)
    - Como parámetro de entrada recibe el archivo tree.out con el árbol sintáctico de Triangle generado por el parse.rs
    - Como resultado se obtiene el árbol en formato .dot en la consola de comandos.




### Instrucciones para Visualizar el Árbol Generado, después de ejecutar el pare.rs

1. **Generar la Estructura en Consola**  
   Cuando se ejecute el tercer comando, se generará en la consola la estructura del archivo .dot.

2. **Copiar el Contenido de la Consola**  
   Seleccionar y copiar el contenido que aparece en la consola.

3. **Pegar en GraphvizOnline**  
   Ingrese al siguiente enlace para visualizar el árbol: [GraphvizOnline](https://dreampuf.github.io/GraphvizOnline/).

4. **Visualizar el Árbol**  
   En la página de GraphvizOnline, pegar el contenido copiado en el área de texto para ver el árbol visualizado.


