use std::{
    path::PathBuf,
    str::FromStr,
    fs::{self, File},
    io::{LineWriter, Write},
    env::args
};

fn main() {
    let data_path = args().nth(1)
        .expect("Data path not provided");
    let result_path = args().nth(2)
        .expect("Result path not provided");

    let mut path = PathBuf::from_str(&data_path)
        .expect("Path not found");

    path.push("News Articles");

    let file = File::create(&result_path)
        .expect("Couldn't create results file");
    let mut file = LineWriter::new(file);

    let categories = path.read_dir() 
        .expect("Failed to read categories");

    categories.into_iter().for_each(|category| {
        let category = fs::read_dir(
            category.expect("Failed to read category").path()
        ).expect("Failed to read category");

        category.into_iter().for_each(|article| {
            let article_path = article.expect("Failed to read article").path();
            let article = fs::read(&article_path)
                .expect("Failed to read article");
            let summary_path = article_path.to_str().unwrap().replace("News Articles", "Summaries");
            let summary = fs::read(summary_path)
                .expect("Failed to read summary");
                
            file.write(b"|").expect("Failed to write file");
            file.write(article.as_slice()).expect("Failed to write file");
            file.write(b"|,|").expect("Failed to write file");
            file.write(summary.as_slice()).expect("Failed to write file");
            file.write(b"|\n").expect("Failed to write file");
            file.flush().expect("Failed to write file")
        });
    });

}
