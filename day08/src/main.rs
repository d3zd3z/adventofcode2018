use std::{
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let codes = get_input()?;

    let tree = Tree::from_codes(&mut codes.iter().cloned());

    println!("result1: {}", tree.metadata_total());
    println!("result2: {}", tree.value());
    Ok(())
}

#[derive(Debug)]
struct Tree {
    children: Vec<Tree>,
    metadata: Vec<usize>,
}

impl Tree {
    // Decode an iterator over codes into a tree.
    fn from_codes<I>(source: &mut I) -> Tree where
        I: Iterator<Item=usize>,
    {
        let nchildren = source.next().unwrap();
        let nmeta = source.next().unwrap();

        let children = (0 .. nchildren).map(|_| Tree::from_codes(source)).collect();
        let meta = (0 .. nmeta).map(|_| source.next().unwrap()).collect();

        Tree{
            children: children,
            metadata: meta,
        }
    }

    /// Get the total metadata.
    fn metadata_total(&self) -> usize {
        self.children.iter().map(|c| c.metadata_total()).sum::<usize>() +
            self.metadata.iter().sum::<usize>()
    }

    /// Get the 'value' as defined in the problem.  If there are no
    /// children, it is the sum of the metadata.  If there are children,
    /// use the metadata as 1-based indices into the children.  Skip any
    /// that aren't valid.
    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum::<usize>()
        } else {
            self.metadata.iter().map(|&m| {
                if m == 0 || m > self.children.len() {
                    0
                } else {
                    self.children[m-1].value()
                }
            }).sum::<usize>()
        }
    }
}

fn get_input() -> Result<Vec<usize>> {
    let f = BufReader::new(File::open("license.txt")?);

    let mut result = vec![];
    let line = f.lines().next().unwrap()?;

    for n in line.split(" ") {
        result.push(n.parse()?);
    }
    Ok(result)
}
