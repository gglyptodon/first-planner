#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
extern crate chrono;
use chrono::Duration;
pub mod workout;
use workout::{calc_paces, PaceCategory, PacePrinter, Workout, WorkoutType};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    let custom_pace_min = url.search().get("min");
    let custom_pace_sec = url.search().get("sec");
    let mut tmp_dur_min: i64 = 5;
    let mut tmp_dur_sec: i64 = 0;
    if let Some(x) = custom_pace_min {
        if let Ok(y) = x.get(0).unwrap_or(&"5".to_string()).parse::<i64>() {
            tmp_dur_min = y;
        }
    }
    if let Some(x) = custom_pace_sec {
        if let Ok(y) = x.get(0).unwrap_or(&"0".to_string()).parse::<i64>() {
            tmp_dur_sec = y;
        }
    }

    Model {
        counter: 0,
        url,
        base_pace: Duration::minutes(tmp_dur_min)
            .checked_add(&Duration::seconds(tmp_dur_sec))
            .unwrap_or_else(|| Duration::minutes(5)),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
    base_pace: chrono::Duration,
    url: Url,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment(i64),
    Decrement(i64),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment(sec) => {
            if let Some(x) = model.base_pace.checked_add(&Duration::seconds(sec)) {
                model.base_pace = x;
                model.counter += 1;
            } else {
            }
        }
        Msg::Decrement(sec) => {
            if let Some(x) = model.base_pace.checked_sub(&Duration::seconds(sec)) {
                model.base_pace = x;
                model.counter -= 1;
            } else {
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn plus_button<T: 'static>() -> Node<T> {
    div![button![
        C!["btn btn-outline-primary"],
        " +1 sec",
        ev(Ev::Click, |_| Msg::Increment(1)),
    ],]
}
fn plus_ten_button<T: 'static>() -> Node<T> {
    div![button![
        C!["btn btn-outline-primary"],
        "+10 sec",
        ev(Ev::Click, |_| Msg::Increment(10)),
    ],]
}

fn minus_button<T: 'static>() -> Node<T> {
    div![button![
        C!["btn btn-outline-primary"],
        " -1 sec",
        ev(Ev::Click, |_| Msg::Decrement(1)),
    ],]
}
fn minus_ten_button<T: 'static>() -> Node<T> {
    div![button![
        C!["btn btn-outline-primary"],
        "-10 sec",
        ev(Ev::Click, |_| Msg::Decrement(10)),
    ],]
}

fn interval_table<T: 'static>(model: &Model) -> Node<T> {
    table![
        C!["table table-bordered table-hover"],
        tr![
            th!["400m"],
            th!["600m"],
            th!["800m"],
            th!["1k"],
            th!["1200m"],
            th!["1600m"],
            th!["2k"],
        ],
        tr![
            td![calc_paces(&model.base_pace, &PaceCategory::M400).print_pace_formatted()],
            td![calc_paces(&model.base_pace, &PaceCategory::M600).print_pace_formatted()],
            td![calc_paces(&model.base_pace, &PaceCategory::M800).print_pace_formatted()],
            td![calc_paces(&model.base_pace, &PaceCategory::K1).print_pace_formatted()],
            td![calc_paces(&model.base_pace, &PaceCategory::M1200).print_pace_formatted()],
            td![calc_paces(&model.base_pace, &PaceCategory::M1600).print_pace_formatted()],
            td![calc_paces(&model.base_pace, &PaceCategory::K2).print_pace_formatted()],
        ],
    ]
}

fn tempo_table<T: 'static>(model: &Model) -> Node<T> {
    table![
        C!["table table-bordered table-hover"],
        tr![th!["Short"], th!["Mid"], th!["Long"],],
        tr![
            td![calc_paces(&model.base_pace, &PaceCategory::ShortTempo).print_pace_formatted()],
            td![calc_paces(&model.base_pace, &PaceCategory::MidTempo).print_pace_formatted()],
            td![calc_paces(&model.base_pace, &PaceCategory::LongTempo).print_pace_formatted()],
        ],
    ]
}
//TODO
fn long_run_table<T: 'static>(model: &Model) -> Node<T> {
    tempo_table(model)
}

/*fn schedule_cells<T: 'static>(week: i32, workout_vec: &Vec<Workout>) -> Node<T> {
    //let interval = workout_vec.get(0).unwrap();
    //let tempo = workout_vec.get(1).unwrap();
    //let long = workout_vec.get(2).unwrap();
    // todo
    let res = tr![
        td![format!("{}", week)] //,
        ];
    res
}*/
fn schedule_table_row<T: 'static>(
    week: i32,
    workouts: &[Workout],
    dummy: &Workout,
    model: &Model,
) -> Node<T> {
    tr![
        td![workouts
            .iter()
            .find(|w| w.week == week && w.workout_type == WorkoutType::Interval)
            .unwrap_or(dummy)
            .show_with_pace(model.base_pace)],
        td![workouts
            .iter()
            .find(|w| w.week == week && w.workout_type == WorkoutType::Tempo)
            .unwrap_or(dummy)
            .show_with_pace(model.base_pace)],
        td![workouts
            .iter()
            .find(|w| w.week == week && w.workout_type == WorkoutType::Long)
            .unwrap_or(dummy)
            .show_with_pace(model.base_pace)]
    ]
}

fn schedule_table<T: 'static>(model: &Model) -> Node<T> {
    let mut workouts: Vec<Workout> = Vec::new();
    workouts.push(Workout::new(
        1,
        "+10-20min WA/CD".to_string(),
        WorkoutType::Interval,
        PaceCategory::M400,
        "8x 400m".to_string(),
    ));

    let dummy = Workout::new(
        0,
        "dummy".to_string(),
        WorkoutType::Interval,
        PaceCategory::K1,
        "dummy".to_string(),
    );
    let r = 1..16;
    let mut wks: Vec<Node<T>> = Vec::new();
    for i in r {
        wks.push(schedule_table_row(i, &workouts, &dummy, model));
    }

    table![C!["table table-bordered"], wks]
}

fn plus_button_group() -> Node<Msg> {
    div![C!["btn-group"], plus_button(), plus_ten_button()]
}
fn minus_button_group() -> Node<Msg> {
    div![C!["btn-group"], minus_button(), minus_ten_button()]
}
fn assembled_view(model: &Model, heading: &str) -> Node<Msg> {
    div![
        style! {
            "background-color" => "white",
            "color" => "black",
            "min-height" => "5rem",
            "line-height" => "5rem",
            "text-align" => "center",
        },
        h2![heading],
        h3![id!["current_date"]],
        Script!["const d = new Date();console.log('<h3>'+d+'</h3>');var x = document.getElementById('current_date'); x.innerHTML= d;"],

        minus_button_group(),
        br![],
        plus_button_group(),
        br![],
        schedule_table(model),
        interval_table(model),
        br![],
        tempo_table(model),
        br![],
       //TODO: long_run_table(model),
    ]
}

fn view(model: &Model) -> Node<Msg> {
    assembled_view(
        model,
        &format!("Base pace: {}", model.base_pace.print_pace_formatted()),
    )
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

// ------ ------
//     Misc
// ------ ------
