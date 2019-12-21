use std::sync::Arc;

use plotlib::page::Page;
use plotlib::scatter;
use plotlib::scatter::Scatter;
use plotlib::line;
use plotlib::line::Line;
use plotlib::style::{Point, Line as OtherLine};
use plotlib::view::ContinuousView;
use rand::distributions::Distribution;
use rand_distr::Normal;

use basic_pbt_demo::{CommonFunctions, Vector};

use crate::pbt_trainer::{PBTTrainer, State, TrainingEvent};

mod pbt_trainer;

const ITERATIONS: i32 = 100;
const STD_DEV: f64 = 0.1;
const WORKERS: i32 = 4;

fn main() {
    // More colours than needed to accommodate more workers
    let colours = vec!["#FF0000", "#00FF00", "#0000FF", "#FF00FF", "#FFFF00", "#00FFFF", "#FFFFFF"];
    let start_vector = Vector { a: 1.0, b: 1.0 };

    let eval_function = Arc::new(CommonFunctions::example_fn);
    let derivative_function = Arc::new(CommonFunctions::example_derivative);
    let actual_function = Arc::new(CommonFunctions::actual_fn);

    let mut pbt = PBTTrainer::new(eval_function, derivative_function, Arc::new(|state: State| {
        let normal = Normal::new(0.0, STD_DEV).expect("Could not create normal distribution");
        let normal_closure = |v: f64| { v + normal.sample(&mut rand::thread_rng()) };
        State {
            theta: state.theta.apply(&normal_closure),
            h: state.h.apply(&normal_closure),
        }
    }), WORKERS, ITERATIONS);

    let results = pbt.start(start_vector, 0.1);

    let mut data = Vec::new();
    let mut transitions = Vec::new();
    for points in results.iter() {
        let mut p = Vec::new();
        let mut e = Vec::new();
        for event in points.points.iter() {
            match event {
                TrainingEvent::Point(v) => p.push((v.a, v.b)),
                TrainingEvent::Exploit(_, v) | TrainingEvent::Explore(_, v) => e.push((v.a, v.b)),
            }
        }
        data.push(p);
        transitions.push(e);
    }

    let mut view1 = ContinuousView::new();
    let mut view2 = ContinuousView::new();
    let mut view3 = ContinuousView::new();

    // Need to preserve these because plot wants a reference to the objects
    let mut data_scatters = Vec::new();
    let mut q_scatters = Vec::new();
    for (i, d) in data.iter().enumerate() {
        let s = Scatter::from_slice(d.as_slice())
            .style(scatter::Style::new().colour(colours[i]));
        data_scatters.push(s);

        let q = d.iter().enumerate().map(|(i, v)| {
            (i as f64, (actual_function)(Vector::from_tuple(v)) as f64)
        });
        let mut qs_as_vector = Vec::new();
        q.for_each(|v| {
            qs_as_vector.push(v);
        });
        let s = Line::new(qs_as_vector.as_slice())
            .style(line::Style::new().colour(colours[i]));
        q_scatters.push(s);
    }

    let mut transition_scatters = Vec::new();
    for (i, d) in transitions.iter().enumerate() {
        let s = Scatter::from_slice(d.as_slice())
            .style(scatter::Style::new().colour(colours[i]));
        transition_scatters.push(s);
    }

    for s in data_scatters.iter() {
        view1 = view1.add(s);
    }

    for s in transition_scatters.iter() {
        view2 = view2.add(s);
    }

    for s in q_scatters.iter() {
        view3 = view3.add(s);
    }


    view1 = view1
        .x_range(0.0, 1.0)
        .y_range(0.0, 1.0)
        .x_label("θ0")
        .y_label("θ1");

    view2 = view2
        .x_range(0.0, 1.0)
        .y_range(0.0, 1.0)
        .x_label("h0")
        .y_label("h1");

    view3 = view3
        .x_range(0.0, ITERATIONS as f64)
        .y_range(-1.0, 1.2)
        .x_label("ITERATIONS")
        .y_label("Q(theta)");

    Page::empty().add_plot(&view1).save("target/theta_scatter.svg").expect("Couldn't make theta plot");
    Page::empty().add_plot(&view2).save("target/h_scatter.svg").expect("Couldn't make h plot");
    Page::empty().add_plot(&view3).save("target/q_lines.svg").expect("Couldn't make q plot");
}
