use std::{collections::HashMap, fs, path::Path, process::Command};

use nom::{
    bytes::complete::take_while1,
    character::complete::{char, line_ending, space0, space1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult, Parser,
};
use petgraph::{
    dot::{Config, Dot},
    graph::{Graph, NodeIndex},
    Directed,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Data>> {
    let identifier = |input| take_while1(|c: char| !c.is_whitespace() && c != ':')(input);

    let line = map(
        (
            identifier,
            (space0, char(':'), space0),
            separated_list1(space1, identifier),
        ),
        |(device, _, outputs): (&str, (_, char, _), Vec<&str>)| Data {
            devices: device.to_string(),
            outputs: outputs.into_iter().map(str::to_string).collect(),
        },
    );

    many1(terminated(line, opt(line_ending))).parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    devices: String,
    outputs: Vec<String>,
}

fn build_graph(data: &[Data]) -> (Graph<String, (), Directed>, HashMap<String, NodeIndex>) {
    let mut graph: Graph<String, (), Directed> = Graph::new();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

    for entry in data {
        let from_idx = *nodes
            .entry(entry.devices.clone())
            .or_insert_with(|| graph.add_node(entry.devices.clone()));

        for output in &entry.outputs {
            let to_idx = *nodes
                .entry(output.clone())
                .or_insert_with(|| graph.add_node(output.clone()));
            graph.add_edge(from_idx, to_idx, ());
        }
    }

    (graph, nodes)
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    let (graph, nodes) = build_graph(&data);

    if let Err(err) = export_graphviz_png(&graph, "graph.png") {
        eprintln!("Failed to export graph PNG: {err}");
    }

    let Some(&start) = nodes.get("you") else {
        return 0;
    };
    let Some(&target) = nodes.get("out") else {
        return 0;
    };

    fn count_paths(
        node: NodeIndex,
        target: NodeIndex,
        graph: &Graph<String, (), Directed>,
        memo: &mut HashMap<NodeIndex, usize>,
        visiting: &mut std::collections::HashSet<NodeIndex>,
    ) -> usize {
        if node == target {
            return 1;
        }

        if let Some(&cached) = memo.get(&node) {
            return cached;
        }

        // Prevent infinite loops in case of cycles.
        if !visiting.insert(node) {
            return 0;
        }

        let total = graph
            .neighbors(node)
            .map(|next| count_paths(next, target, graph, memo, visiting))
            .sum();

        visiting.remove(&node);
        memo.insert(node, total);
        total
    }

    count_paths(
        start,
        target,
        &graph,
        &mut HashMap::new(),
        &mut std::collections::HashSet::new(),
    )
}

fn export_graphviz_dot(
    graph: &Graph<String, (), Directed>,
    path: impl AsRef<Path>,
) -> std::io::Result<()> {
    let dot = format!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]));
    fs::write(path, dot)
}

fn export_graphviz_png(
    graph: &Graph<String, (), Directed>,
    png_path: impl AsRef<Path>,
) -> std::io::Result<()> {
    let png_path = png_path.as_ref();
    let dot_path = png_path.with_extension("dot");
    export_graphviz_dot(graph, &dot_path)?;

    let status = Command::new("dot")
        .arg("-Tpng")
        .arg(&dot_path)
        .arg("-o")
        .arg(png_path)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to run graphviz 'dot' to produce PNG",
        ))
    }
}

fn main() {
    let input = read_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_run1() {
        let input = read_input(Some(indoc!(
            r"
            aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 5);
    }
}
