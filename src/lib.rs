mod graph;

use std::{collections::HashMap, env, error::Error, fs};

use graph::{
    // bfs,
    dfs,
    {Edge, Graph, Vertex},
};

pub struct Config {
    pub input_file: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Config {
        args.next();

        let input_file = match args.next() {
            Some(arg) => arg,
            None => {
                println!("Nome de arquivo de entrada não informado, lendo arquivo padrão");
                "casoleonardo10.txt".to_string()
            }
        };

        Config { input_file }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.input_file)?;
    let (graph, name_to_vertex_map) = parse_input_content(&contents);

    let mut two_flavour_count = 0;
    let mut three_flavour_count = 0;

    for base_key in name_to_vertex_map.keys() {
        let root_vertex = name_to_vertex_map.get(base_key).unwrap();
        for compare_key in name_to_vertex_map.keys() {
            if base_key == compare_key {
                continue;
            }
            let target = name_to_vertex_map.get(compare_key).unwrap();
            if let Some(path) = dfs::depth_first_search(&graph, *root_vertex, *target) {
                // if let Some(path) = bfs::breadth_first_search(&graph, *root_vertex, *target) {
                two_flavour_count += 1;
                if path.len() >= 3 {
                    three_flavour_count += 1;
                }
            }
        }
    }

    println!(
        "Dois sabores: {} / Três sabores: {}",
        two_flavour_count, three_flavour_count
    );
    Ok(())
}

fn parse_input_content(contents: &String) -> (Graph, HashMap<String, Vertex>) {
    let mut lines = contents.lines();
    let mut name_to_vertex_map: HashMap<String, Vertex> = HashMap::new();
    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    while let Some(next_line) = lines.next() {
        let from_to_vector: Vec<&str> = next_line
            .split("->")
            .into_iter()
            .map(|n| n.trim())
            .collect();
        let (from, to) = (from_to_vector[0], from_to_vector[1]);

        let from = parse_vertex(from, &mut name_to_vertex_map, &mut vertices);
        let to = parse_vertex(to, &mut name_to_vertex_map, &mut vertices);
        let edge = Edge::from((from, to));
        edges.push(edge);
    }
    (Graph::new(vertices, edges), name_to_vertex_map)
}

fn parse_vertex(key: &str, map: &mut HashMap<String, Vertex>, vertices: &mut Vec<Vertex>) -> u32 {
    let vertex = if let Some(v) = map.get(key) {
        v
    } else {
        let v = Vertex::from(map.len() as u32);
        vertices.push(v);
        map.insert(key.to_string(), v);
        map.get(key).unwrap()
    };
    vertex.value()
}
