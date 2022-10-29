use itertools::iproduct;

pub struct SimulationContext {
    num_steps: usize,
}

impl SimulationContext {
    pub fn new(num_steps: usize) -> Self {
        Self { num_steps }
    }
}

type MapElements3D = Vec<Vec<Vec<bool>>>;
type MapElements4D = Vec<MapElements3D>;

#[derive(Clone)]
pub struct Data3d {
    dimension: usize,
    elements: MapElements3D,
}

impl Data3d {
    pub fn new(dimension: usize, elements: MapElements3D) -> Self {
        Self {
            dimension,
            elements,
        }
    }
}

#[derive(Clone)]
pub struct Data4d {
    dimension: usize,
    elements: MapElements4D,
}

impl Data4d {
    pub fn new(dimension: usize, elements: MapElements4D) -> Self {
        Self {
            dimension,
            elements,
        }
    }
}

fn run_iteration_3d(
    prev_map: &[Vec<Vec<bool>>],
    start: usize,
    end: usize,
    delta: &[(i64, i64, i64)],
) -> MapElements3D {
    let mut next_map_elements = prev_map.to_owned();

    for (x, y, z) in iproduct!(start..end, start..end, start..end) {
        let cur_state = prev_map[x][y][z];

        let alive = delta
            .iter()
            .filter(|(dx, dy, dz)| {
                let x = ((x as i64) + dx) as usize;
                let y = ((y as i64) + dy) as usize;
                let z = ((z as i64) + dz) as usize;

                prev_map[x][y][z]
            })
            .count();

        if cur_state && alive != 2 && alive != 3 {
            next_map_elements[x][y][z] = false;
        } else if !cur_state && alive == 3 {
            next_map_elements[x][y][z] = true;
        }
    }

    next_map_elements
}

fn run_iteration_4d(
    prev_map: &[Vec<Vec<Vec<bool>>>],
    start: usize,
    end: usize,
    delta: &[(i64, i64, i64, i64)],
) -> MapElements4D {
    let mut next_map_elements = prev_map.to_owned();

    for (x, y, z, w) in iproduct!(start..end, start..end, start..end, start..end) {
        let cur_state = prev_map[x][y][z][w];

        let alive = delta
            .iter()
            .filter(|(dx, dy, dz, dw)| {
                let x = ((x as i64) + dx) as usize;
                let y = ((y as i64) + dy) as usize;
                let z = ((z as i64) + dz) as usize;
                let w = ((w as i64) + dw) as usize;

                prev_map[x][y][z][w]
            })
            .count();

        if cur_state && alive != 2 && alive != 3 {
            next_map_elements[x][y][z][w] = false;
        } else if !cur_state && alive == 3 {
            next_map_elements[x][y][z][w] = true;
        }
    }

    next_map_elements
}

fn count_alive_cells(elem: &[Vec<Vec<bool>>]) -> usize {
    let mut res = 0;

    for (x, y, z) in iproduct!(0..elem.len(), 0..elem.len(), 0..elem.len()) {
        if elem[x][y][z] {
            res += 1;
        }
    }

    res
}

pub fn run_simulation_steps_3d(map_data: &Data3d, context: &SimulationContext) -> usize {
    let start = 1;
    let end = map_data.dimension - 2;

    let delta = build_delta_3d();

    let mut state = map_data.elements.clone();
    for _ in 0..context.num_steps {
        state = run_iteration_3d(&state, start, end, &delta);
    }

    count_alive_cells(&state)
}

pub fn run_simulation_steps_4d(map_data: &Data4d, context: &SimulationContext) -> usize {
    let start = 1;
    let end = map_data.dimension - 2;

    let delta = build_delta_4d();

    let mut state = map_data.elements.clone();
    for _ in 0..context.num_steps {
        state = run_iteration_4d(&state, start, end, &delta);
    }

    state.iter().map(|it| count_alive_cells(it)).sum()
}

fn build_delta_3d() -> Vec<(i64, i64, i64)> {
    let mut res = Vec::new();

    for (x, y, z) in iproduct!(-1..=1, -1..=1, -1..=1) {
        if x == 0 && y == 0 && z == 0 {
            continue;
        }

        res.push((x, y, z));
    }

    res
}

fn build_delta_4d() -> Vec<(i64, i64, i64, i64)> {
    let mut res = Vec::new();

    for (x, y, z, w) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
        if x == 0 && y == 0 && z == 0 && w == 0 {
            continue;
        }

        res.push((x, y, z, w));
    }

    res
}
