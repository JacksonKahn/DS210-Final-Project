//Collaborator: None

mod linecount;
mod read_csv;

extern crate rand;
use rand::Rng;
use rand::thread_rng;
fn main(){
    //Testcode to demonstrate that the algorithm works
    let mut testmat = Vec::new();
    testmat.push(vec![1,2]);
    testmat.push(vec![0,2]);
    testmat.push(vec![0,1]);
    let used:Vec<usize> = vec![0,1,2];
    println!("{:?}", average_distance(testmat, 3, used));

    let streamer_count = linecount::csv_length("Twitch_Gamers_Dataset/large_twitch_features.csv", true);
    println!("There are {:?} streamers in the dataset.", streamer_count);
    let mut streamers_info = read_csv::read_in_streamers_csv(streamer_count);
    let connections_count = linecount::csv_length("Twitch_Gamers_Dataset/large_twitch_edges.csv", true);
    println!("There are {:?} undirected connections between streamers.", connections_count);
    let mut streamer_connections = read_csv::read_in_edges_csv(connections_count);
    let picked = pick_k(&mut streamers_info, 1);
    println!("{:?}", picked);
    let mut all_distances: Vec<Vec<i32>> = Vec::with_capacity(picked.len());
    let mut avg_distance: Vec<f64> = Vec::new();
    (all_distances, avg_distance) =  average_distance(streamer_connections, connections_count as usize, picked);
    println!("{:?}", avg_distance);
    println!("{:?}", all_distances);
}

fn appstart(mut queue: Vec<i32>, new : &Vec<i32>, check : &mut Vec<i32>) -> (Vec<i32>, Vec<i32>){
    for x in 0..new.len(){
        if check[new[x] as usize] == 0 {
            queue.insert(0, new[x]);
            check[new[x] as usize] = 1;
        }
    }
    return (queue, check.to_vec());
}

fn add_distances(mut distances : Vec<i32>, new : &Vec<i32>, current : usize) -> Vec<i32>{
    let new_dist = distances[current] + 1;
    for x in 0..new.len(){
        //Since a node can be accesssed from multiple other nodes in the graph, need to check if it has been linked beforehand
        if distances[new[x] as usize] == -1{
            distances[new[x] as usize] = new_dist;
        }
    }
    return distances;
}

fn pick_k(nodes : &mut Vec<Vec<i32>>, k :usize) -> Vec<usize> {
    let mut chosen:Vec<usize> = Vec::with_capacity(k);
    for _x in 0..k {
        let choice = thread_rng().gen_range(0..nodes.len());
        chosen.push(nodes[choice][0] as usize);
        nodes.remove(choice);
    }
    return chosen;
}

fn average_distance(matrix : Vec<Vec<i32>>, num_nodes : usize, used_nodes : Vec<usize>) -> (Vec<Vec<i32>>, Vec<f64>) {
    let mut sums :Vec<Vec<i32>> = vec![vec![0,0,0]; used_nodes.len()];
    //With this particular data set I can skip some error checking since all of the points are reachable from each other
    //It is extremely computationally expensive to run over every node, so I make this part ajustable.
    //I include this counter to show that there is progess being made.
    let mut count = 0;
    for x in 0..used_nodes.len() {
        //Sets the starting vertex
        let mut current_vertex = used_nodes[x] ;
        //Sets up the list of distances relative to the starting node, which is distance 0
        let mut relative_dists :Vec<i32> = vec![-1; num_nodes];
        relative_dists[current_vertex] = 0;
        //Keeping track of all of the visitied nodes in this particular cycle
        let mut visited : Vec<i32> = vec![0; num_nodes];
        //This is the queue to be used in the rest of the breadth first search, it keeps track of the the important 
        let mut queue : Vec<i32> = Vec::with_capacity(num_nodes);
        let mut enqueued : Vec<i32> = vec![0; num_nodes];
        visited[current_vertex] = 1;
        (queue, enqueued) = appstart(queue, &matrix[current_vertex], &mut enqueued);
        relative_dists = add_distances(relative_dists, &matrix[current_vertex], current_vertex);
        //BFS continues until the queue is empty, aka when there are no more unexplored nodes
        while !queue.is_empty() {
            //Checks if the vertex has been visited before, if it has goes to the next one in the queue
            if visited[current_vertex] == 1{ 
                current_vertex = queue.pop().unwrap() as usize;
                continue;
            }
            //Otherwise, marks the vertex as visited, adds its linked nodes to the queue and assigns distances to the linked nodes
            visited[current_vertex] = 1;
            //Only continues to add to the queue until distance 2 from the starting node
            if relative_dists[current_vertex] < 3 {
                (queue, enqueued) = appstart(queue, &matrix[current_vertex], &mut enqueued);
                relative_dists = add_distances(relative_dists, &matrix[current_vertex], current_vertex);
            }
            current_vertex = queue.pop().unwrap() as usize; 
            count += 1;
            if count%1000 == 0{
                println!("{:?}", count);
            }
        }
        //Accounting for all of the distances relative to each starting point
        for y in 0..num_nodes{
            if relative_dists[y] != -1 {
                let distance = relative_dists[y as usize] as usize;
                if sums[x].len() == distance{
                    sums[x].push(1);
                }
                else{
                    sums[x][distance] += 1;
                }
            }
        }
    }
    //Averaging the relative distances for all other nodes, for each node in the dataset
    let mut finalcos :Vec<f64> = Vec::with_capacity(10);
    for i in 0..sums[0].len(){
        let mut colsum = 0;
        for j in 0..sums.len(){
           colsum += sums[j][i];
        }
        finalcos.push((colsum as f64)/(used_nodes.len() as f64));
    }
    return (sums, finalcos);
}