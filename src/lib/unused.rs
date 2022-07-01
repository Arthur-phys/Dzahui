// Calculate distance given a graph representation of a figure
fn max_distance(graph: &HashMap<u32,HashMap<u32,u32>>, vertices: &Vec<f64>) -> f64 {
    // NOT USED ANYMORE
    // Algorithm to acquire maximum length between a graph representation of plane figure
    let mut distance = 0.0;
    for index in graph.keys() {
        // Obtain nodes connected to specified index
        let nodes_connected_to_index = graph.get(index).unwrap();
        for i in nodes_connected_to_index.keys() {
            let beg_index = (3*index) as usize;
            let beg_i = (3*i) as usize;
            let node_index = [vertices[beg_index],vertices[beg_index + 1],vertices[beg_index + 2]];
            let node_i = [vertices[beg_i],vertices[beg_i + 1],vertices[beg_i + 2]];
            let i_index_distance = Mesh::distance(node_index, node_i);
            if i_index_distance > distance {
                distance = i_index_distance;
            }
        }
    }
    distance
}

// Euclidean distance
fn distance(point: [f64;3], point2: [f64;3]) -> f64 {
    let mut d = 0.0;
    for j in 0..2 {
        d += (point[j] - point2[j]).powf(2.0);
    }
    d.sqrt()
}