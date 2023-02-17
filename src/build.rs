use std::path::PathBuf;
use aiken::with_project;

fn build(project_folder: &str) {
    let path_buf = PathBuf::from(project_folder);
    println!("{:?}", path_buf.exists());
    with_project(Option::Some(path_buf), |p| p.build(false));
}

#[cfg(test)]
mod tests {
    use crate::build::build;

    // #[test]
    // fn test_build() {
    //     build("/Users/satya/work/aiken-samples/helloworld");
    // }
}
