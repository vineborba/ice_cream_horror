use std::collections::HashSet;
use std::collections::VecDeque;

use super::{Graph, Vertex};

#[allow(dead_code)]
pub fn breadth_first_search(graph: &Graph, root: Vertex, target: Vertex) -> Option<Vec<u32>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut queue = VecDeque::new();
    let mut history: Vec<u32> = Vec::new();

    visited.insert(root);
    queue.push_back(root);
    // Executa o loop enquanto houver elementos na fila de vértices
    while let Some(current_vertex) = queue.pop_front() {
        // Adiciona o vértice atual no histórico
        history.push(current_vertex.value());

        // Caso tenha encontrado o alvo, retorna o caminho.
        if current_vertex == target {
            return Some(history);
        }

        // Procurando vizinhos que não foram ainda visitados.
        for neighbor in current_vertex.neighbors(graph) {
            // Insere na lista de visitados
            if visited.insert(neighbor) {
                // Adiciona o vizinho na fila somente se foi inserido na
                // lista de visitados
                queue.push_back(neighbor);
            }
        }
    }

    // Todos vértices visitados, alvo não foi encontrado.
    None
}
