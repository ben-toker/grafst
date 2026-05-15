use crate::graph::Graph;

const ITERS: usize = 100;
const W: f64 = 10.0;
const H: f64 = 10.0;
const C: f64 = 1.0;
const EPSILON: f64 = 0.01;

pub fn layout(graph: &mut Graph) {
    let n = graph.nodes.len();
    if n == 0 {
        return;
    }

    let area = W * H;
    let k = C * (area / n as f64).sqrt();

    // Initial positions on a unit circle of radius 2.
    let radius = 2.0;
    for (i, node) in graph.nodes.iter_mut().enumerate() {
        let theta = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
        node.position = (radius * theta.cos(), radius * theta.sin());
    }

    let mut t: f64 = W / 10.0;
    let cooling = 0.95;

    let mut disp: Vec<(f64, f64)> = vec![(0.0, 0.0); n];

    for _ in 0..ITERS {
        for d in disp.iter_mut() {
            *d = (0.0, 0.0);
        }

        // Repulsion: every pair.
        for u in 0..n {
            for v in (u + 1)..n {
                let (ux, uy) = graph.nodes[u].position;
                let (vx, vy) = graph.nodes[v].position;
                let dx = vx - ux;
                let dy = vy - uy;
                let dist = (dx * dx + dy * dy).sqrt().max(EPSILON);
                let force = k * k / dist;
                let fx = dx / dist * force;
                let fy = dy / dist * force;
                disp[v].0 += fx;
                disp[v].1 += fy;
                disp[u].0 -= fx;
                disp[u].1 -= fy;
            }
        }

        // Attraction: along edges.
        for edge in &graph.edges {
            let (ux, uy) = graph.nodes[edge.from].position;
            let (vx, vy) = graph.nodes[edge.to].position;
            let dx = vx - ux;
            let dy = vy - uy;
            let dist = (dx * dx + dy * dy).sqrt().max(EPSILON);
            let force = dist * dist / k;
            let fx = dx / dist * force;
            let fy = dy / dist * force;
            disp[edge.to].0 -= fx;
            disp[edge.to].1 -= fy;
            disp[edge.from].0 += fx;
            disp[edge.from].1 += fy;
        }

        // Apply displacement, capped by temperature.
        for i in 0..n {
            let (dx, dy) = disp[i];
            let mag = (dx * dx + dy * dy).sqrt().max(EPSILON);
            let capped = mag.min(t);
            let nx = graph.nodes[i].position.0 + dx / mag * capped;
            let ny = graph.nodes[i].position.1 + dy / mag * capped;
            graph.nodes[i].position = (nx, ny);
        }

        t *= cooling;
    }
}
