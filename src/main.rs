use basic_pbt_demo::{CommonFunctions, Vector};
use crate::pbt_trainer::{PBTTrainer, State, TrainingEvent};
use std::sync::Arc;

use plotlib::view::{View, ContinuousView};
use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::Point;
use plotlib::page::Page;

mod pbt_trainer;

fn main() {
  let colours = vec!["#FF0000", "#00FF00", "#0000FF", "#FF00FF", "#FFFF00", "#00FFFF", "#FFFFFF"];
  let start_vector = Vector { a: 0.5, b: 0.5 };

  let eval_function = Arc::new(CommonFunctions::example_fn);
  let derivative_function = Arc::new(CommonFunctions::example_derivative);
  let actual_function = Arc::new(CommonFunctions::actual_fn);

  let mut pbt = PBTTrainer::new(eval_function, derivative_function, Arc::new(|state: State| {
    state
  }), 2, 100);

  let mut results = pbt.start(start_vector, 0.01);
  for result in results.iter() {
    println!("Results were {:?}", result);
  }

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

  // Need to preserve these because plot wants a reference to the objects
  let mut data_scatters = Vec::new();
  for (i, d)  in data.iter().enumerate() {
    let s = Scatter::from_slice(d.as_slice())
        .style(scatter::Style::new().colour(colours[i]));
    data_scatters.push(s);
  }

  for s in data_scatters.iter() {
    view1 = view1.add(s);
  }

  let mut transition_scatters = Vec::new();
  for (i, d) in transitions.iter().enumerate() {
    let s = Scatter::from_slice(d.as_slice())
        .style(scatter::Style::new().colour(colours[i]));
    transition_scatters.push(s);
  }

  for s in transition_scatters.iter() {
    view2 = view2.add(s);
  }

  view1 = view1
      .x_range(0.0,1.0)
      .y_range(0.0, 1.0)
      .x_label("θ0")
      .y_label("θ1");

  view2 = view2
      .x_range(0.0, 1.0)
      .y_range(0.0, 1.0)
      .x_label("h0")
      .y_label("h1");

  let page = Page::empty().add_plot(&view1).save("target/theta_scatter.svg");
  let page2 = Page::empty().add_plot(&view2).save("target/h_scatter.svg");

}
