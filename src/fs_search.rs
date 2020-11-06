use ignore::Walk;
use ignore::WalkBuilder;
use ignore::WalkParallel;

pub async fn search(input: String) -> Vec<String> {
    let pattern = format!("{}", input);

    let search_path = dirs::home_dir()
        .unwrap();

    println!("New file search with pattern {} in {:?}", &input, &search_path);

    let mut paths = vec![];
    let walk = WalkBuilder::new(search_path)
        .hidden(false)
        .build();

    for result in walk {
        println!("Found match {:?}", &result);
        if let Ok(entry) = result {
            paths.push(entry.path().to_string_lossy().to_string());
        }
    }

    paths
}
