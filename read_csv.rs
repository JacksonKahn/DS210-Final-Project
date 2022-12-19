//Collaborator: None
extern crate std;
use std::fs::File;

extern crate csv;
use csv::Reader;

pub fn read_in_edges_csv(n : i32) -> Vec<Vec<i32>> {
    // Build the CSV reader and iterate over each record.
    //I use .expect instead of the question mark operator which finds the value from the retured option enum.
    let mut rdr: Reader<File> = csv::Reader::from_path("Twitch_Gamers_Dataset/large_twitch_edges.csv").expect("Unable to open file");
    // Setting up an interator which can move through the csv reader records
    let mut iter = rdr.records();
    //Setting up the array of edges
    let mut streamer_edges : Vec<Vec<i32>> = vec![Vec::new(); n as usize];
    //Skipping header
    iter.next();
    for _x in 0..n{
        let lineoption = iter.next();
        let line = lineoption.expect("Cannot find values").unwrap();
        //Important to specify the type of varaible that we are trying to parse out of the &str
        let streamer1 = line.get(0).unwrap().parse::<i32>().unwrap();
        let streamer2 = line.get(1).unwrap().parse::<i32>().unwrap();
        streamer_edges[streamer1 as usize].push(streamer2);
        streamer_edges[streamer2 as usize].push(streamer1);
    }
    return streamer_edges;
}

pub fn read_in_streamers_csv(n : i32) -> Vec<Vec<i32>> {
    // Build the CSV reader and iterate over each record.
    let mut features_reader: Reader<File> = csv::Reader::from_path("Twitch_Gamers_Dataset/large_twitch_features.csv").expect("Unable to open file");
    // Setting up an interator which can move through the csv reader records
    let mut features = features_reader.records();
    let mut streamers : Vec<Vec<i32>> = Vec::with_capacity(n as usize);
    //The csv reader returns a list of string records, which iter can parse through. To access the records themselves, .unwrap() is needed
    //Can just use a while statement or a loop{} function to run through the rest of the values until the end!, dont need to read csv twice!
    features.next();
    for _x in 0..n {
        let lineoption = features.next();
        let line = lineoption.expect("Cannot find values").unwrap();
        //Important to specify the type of varaible that we are trying to parse out of the &str
        let views = line.get(0).unwrap().parse::<i32>().unwrap();
        let id = line.get(5).unwrap().parse::<i32>().unwrap();
        streamers.push(vec![id, views])
    }
    return streamers;
}