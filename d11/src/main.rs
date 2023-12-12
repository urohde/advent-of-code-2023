use std::{fs::*, io::*, path::Path};

#[derive(Debug, Clone)]
struct Dimension {
    space: Vec<Vec<String>>,
    galaxies: Vec<Galaxy>,
    expand_rows: Vec<usize>,
    expand_columns: Vec<usize>,
}

impl Dimension {
    fn print_space(&self) {
        println!("space: {}x{}", self.space[0].len(), self.space.len());
        for line in self.space.iter() {
            for word in line {
                print!("{}", word);
            }
            print!("\n");
        }
    }

    fn print_galaxies(&self) {
        for galaxy in self.galaxies.iter() {
            println!(
                "Galaxy {} ({},{})",
                galaxy.name, galaxy.location.0, galaxy.location.1
            );
        }
    }

    fn print_expand(&self) {
        println!("expand_columns: {:?}", self.expand_columns);
        println!("expand_rows: {:?}", self.expand_rows);
    }

    fn print(&self) {
        self.print_space();
        self.print_galaxies();
        self.print_expand();
    }

    fn expand(&mut self) {
        self.expand_space();
        self.expand_galaxies();
    }

    fn expand_space(&mut self) {
        for (i, row) in self.expand_rows.iter().enumerate() {
            let new_row = self.space[*row + i].clone();
            self.space.insert(*row + i, new_row)
        }

        for row in self.space.iter_mut() {
            for (j, col) in self.expand_columns.iter().enumerate() {
                row.insert(*col + j, ".".to_string());
            }
        }
    }

    fn expand_galaxies(&mut self) {
        for galaxy in self.galaxies.iter_mut() {
            for (j, col) in self.expand_columns.iter().enumerate() {
                if galaxy.location.0 > *col {
                    galaxy.location.0 += 1;
                }
            }
            for (i, row) in self.expand_rows.iter().enumerate() {
                if galaxy.location.1 > *row {
                    galaxy.location.1 += 1;
                }
            }
        }
    }

    fn get_distances(&self)-> Vec<Distance> {
        let mut distances: Vec<Distance> = Vec::new();
        for g1 in self.galaxies.iter() {
            for g2 in self.galaxies.iter() {
                if g2.name == g1.name {
                    continue;
                }
                if distances
                    .iter()
                    .find(|d| d.from == g2.name && d.to == g1.name)
                    .is_none()
                {
                    distances.push(Distance {
                        from: g1.name.clone(),
                        to: g2.name.clone(),
                        distance: g1.location.0.abs_diff(g2.location.0)
                            + g1.location.1.abs_diff(g2.location.1),
                    })
                }
            }
        }

        return distances;
    }
}

#[derive(Debug, Clone)]
struct Galaxy {
    name: String,
    location: (usize, usize),
}

#[derive(Debug)]
struct Distance {
    from: String,
    to: String,
    distance: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Reading file {}", args[1]);

    let input_dimension = parse_space(read_lines(&args[1]).unwrap());

    println!("--- initial ---");
    input_dimension.print();

    println!("--- expansion ---");

    let mut new_dimension = input_dimension.clone();
    new_dimension.expand();
    new_dimension.print();

    println!("--- distances ---");

    let distances = new_dimension.get_distances();

    for d in distances.iter() {
        print!("{} -> {} = {} \n", d.from, d.to, d.distance);
    }

    let sum = distances.iter().fold(0, |acc, d| acc + d.distance);
    println!("--- sum ---");
    println!("pairs: {}", distances.len());
    println!("sum: {}", sum);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn parse_space(lines: Lines<BufReader<File>>) -> Dimension {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut space: Vec<Vec<String>> = Vec::new();

    let mut no_expand_columns: Vec<usize> = Vec::new();
    let mut expand_rows: Vec<usize> = Vec::new();

    for (i, line) in lines.into_iter().enumerate() {
        if let Ok(l) = line {
            space.push(Vec::new());
            let mut empty_row = true;

            for (j, word) in l.split("").enumerate() {
                match word {
                    "#" => {
                        let y = i;
                        let x = j - 1;
                        galaxies.push(Galaxy {
                            name: (galaxies.len() + 1).to_string(),
                            location: (x, y),
                        });
                        space[i].push(galaxies.len().to_string());
                        empty_row = false;
                        if !no_expand_columns.contains(&x) {
                            no_expand_columns.push(x);
                        }
                    }
                    "." => {
                        space[i].push(".".to_string());
                    }
                    _ => {}
                };
            }
            if empty_row {
                expand_rows.push(i);
            }
        }
    }

    let expand_columns: Vec<usize> = (0..space[0].len())
        .filter(|x| !no_expand_columns.contains(x))
        .collect();

    return Dimension {
        space,
        galaxies,
        expand_rows,
        expand_columns,
    };
}
