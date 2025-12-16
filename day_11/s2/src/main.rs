use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    process::Command,
};

use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    character::complete::{char, line_ending, space0, space1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
};
use petgraph::{
    Directed,
    dot::{Config, Dot},
    graph::{Graph, NodeIndex},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PathState {
    BeforeFft,
    AfterFft,
    AfterDac,
}

struct PathCounter<'a> {
    graph: &'a Graph<String, (), Directed>,
    target: NodeIndex,
    fft: NodeIndex,
    dac: NodeIndex,
    memo: HashMap<(NodeIndex, PathState), usize>,
    visiting: HashSet<(NodeIndex, PathState)>,
}

impl<'a> PathCounter<'a> {
    fn new(
        graph: &'a Graph<String, (), Directed>,
        target: NodeIndex,
        fft: NodeIndex,
        dac: NodeIndex,
    ) -> Self {
        Self {
            graph,
            target,
            fft,
            dac,
            memo: HashMap::new(),
            visiting: HashSet::new(),
        }
    }

    fn advance_state(&self, state: PathState, node: NodeIndex) -> PathState {
        match state {
            PathState::BeforeFft if node == self.fft => PathState::AfterFft,
            PathState::AfterFft if node == self.dac => PathState::AfterDac,
            _ => state,
        }
    }

    fn count(&mut self, node: NodeIndex, state: PathState) -> usize {
        let state = self.advance_state(state, node);

        if node == self.target {
            return match state {
                PathState::AfterDac => 1,
                _ => 0,
            };
        }

        if let Some(&cached) = self.memo.get(&(node, state)) {
            return cached;
        }

        if !self.visiting.insert((node, state)) {
            return 0;
        }

        let total: usize = self
            .graph
            .neighbors(node)
            .map(|next| self.count(next, state))
            .sum();

        self.visiting.remove(&(node, state));
        self.memo.insert((node, state), total);
        total
    }
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    let (graph, nodes) = build_graph(&data);

    if let Err(err) = export_graphviz_png(&graph, "graph.png") {
        eprintln!("Failed to export graph PNG: {err}");
    }

    let Some(&start) = nodes.get("svr") else {
        return 0;
    };
    let Some(&target) = nodes.get("out") else {
        return 0;
    };
    let Some(&fft) = nodes.get("fft") else {
        return 0;
    };
    let Some(&dac) = nodes.get("dac") else {
        return 0;
    };

    let mut counter = PathCounter::new(&graph, target, fft, dac);
    counter.count(start, PathState::BeforeFft)
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
        Err(std::io::Error::other(
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
            svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 2);
    }
}
