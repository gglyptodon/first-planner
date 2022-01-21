use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{write, Formatter};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum WorkoutType {
    Interval,
    Tempo,
    Long,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workout {
    pub week: i32,
    pub description: String,
    pub workout_type: WorkoutType,
    pub pace_category: PaceCategory,
    pub distance: String, //eg 2k
}

impl fmt::Display for Workout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "w{} : {} {}", self.week, self.description, self.distance,)
    }
}

impl fmt::Display for WorkoutType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            WorkoutType::Interval => {
                write!(f, "Interval")
            }
            WorkoutType::Tempo => {
                write!(f, "Tempo")
            }
            WorkoutType::Long => {
                write!(f, "Long")
            }
        }
    }
}

impl Workout {
    pub fn new(
        week: i32,
        description: String,
        workout_type: WorkoutType,
        pace_category: PaceCategory,
        distance: String,
    ) -> Self {
        Self {
            week,
            description,
            workout_type,
            pace_category,
            distance,
        }
    }
    pub fn show_with_pace(&self, d: Duration) -> String {
        format!(
            "w{} : {} {} @ {}",
            self.week,
            self.description,
            self.distance,
            calc_paces(&d, &self.pace_category).print_pace_formatted()
        )
    }
}

pub fn calc_paces(base_pace: &chrono::Duration, ps: &PaceCategory) -> chrono::Duration {
    let mut result = base_pace.checked_add(&chrono::Duration::seconds(0));
    match ps {
        PaceCategory::M400 => {
            result = base_pace.checked_sub(&chrono::Duration::seconds(37));
        }
        PaceCategory::M600 => {
            result = base_pace.checked_sub(&chrono::Duration::seconds(34));
        }
        PaceCategory::M800 => {
            result = base_pace.checked_sub(&chrono::Duration::seconds(31));
        }
        PaceCategory::K1 => {
            result = base_pace.checked_sub(&chrono::Duration::seconds(29));
        }
        PaceCategory::M1200 => {
            result = base_pace.checked_sub(&chrono::Duration::seconds(27));
        }
        PaceCategory::M1600 => {
            result = base_pace.checked_sub(&chrono::Duration::seconds(24));
        }
        PaceCategory::K2 => {
            result = base_pace.checked_sub(&chrono::Duration::seconds(21));
        }
        PaceCategory::ShortTempo => {
            result = base_pace.checked_sub(&chrono::Duration::seconds(0));
        }
        PaceCategory::MidTempo => {
            result = base_pace.checked_add(&chrono::Duration::seconds(9));
        }
        PaceCategory::LongTempo => {
            result = base_pace.checked_add(&chrono::Duration::seconds(18));
        }
        PaceCategory::Long => {
            result = base_pace.checked_add(&chrono::Duration::seconds(30)); //todo
        }
    }
    if let Some(x) = result {
        x
    } else {
        *base_pace // .clone()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PaceCategory {
    M400,
    M600,
    M800,
    K1,
    M1200,
    M1600,
    K2,
    ShortTempo,
    MidTempo,
    LongTempo,
    Long,
}

pub trait PacePrinter {
    fn print_pace_formatted(&self) -> String;
}
impl PacePrinter for chrono::Duration {
    fn print_pace_formatted(&self) -> String {
        if self.num_minutes() > 60 {
            "hours...".to_string()
        } else {
            let minutes_to_display = self.num_minutes();
            let dur_submin: chrono::Duration;
            if let Some(x) = self.checked_sub(&Duration::minutes(self.num_minutes())) {
                dur_submin = x
            } else {
                dur_submin = chrono::Duration::seconds(0)
            }

            let seconds_to_display = dur_submin.num_seconds();
            format!("{:02}:{:02}min/km", minutes_to_display, seconds_to_display)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaggedWorkout {
    pub week: i32,
    pub description: String,
    pub workout_type: WorkoutType,
    pub pace_category: PaceCategory,
    pub distance: String,
    pub tag: String,
}
impl TaggedWorkout {
    //todo: refactor
    pub fn new(workout: Workout, tag: String) -> Self {
        TaggedWorkout {
            week: workout.week,
            description: workout.description,
            workout_type: workout.workout_type,
            pace_category: workout.pace_category,
            distance: workout.distance,
            tag,
        }
    }
}
