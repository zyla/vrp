//! This module provides default strategies.

use super::*;
use crate::json::problem::*;
use crate::json::Location;
use crate::{format_time, parse_time};

const START_DAY: &str = "2020-07-04T00:00:00Z";

pub fn default_time_plus_offset(offset: i32) -> String {
    format_time(parse_time(&START_DAY.to_string()) + from_hours(offset).as_secs_f64())
}

pub fn default_job_single_day_time_windows() -> impl Strategy<Value = Vec<Vec<String>>> {
    generate_multiple_time_windows_fixed(
        START_DAY,
        vec![from_hours(9), from_hours(14)],
        vec![from_hours(2), from_hours(4)],
        1..3,
    )
}

pub fn default_job_place_prototype() -> impl Strategy<Value = JobPlace> {
    simple_job_place_prototype(
        generate_simple_locations(1..100),
        generate_durations(10..20),
        generate_no_tags(),
        default_job_single_day_time_windows(),
    )
}

pub fn default_delivery_prototype() -> impl Strategy<Value = JobVariant> {
    delivery_job_prototype(default_job_place_prototype(), generate_simple_demand(1..5), generate_no_skills())
}

pub fn default_pickup_prototype() -> impl Strategy<Value = JobVariant> {
    pickup_job_prototype(default_job_place_prototype(), generate_simple_demand(1..5), generate_no_skills())
}

pub fn default_pickup_delivery_prototype() -> impl Strategy<Value = JobVariant> {
    pickup_delivery_job_prototype(
        default_job_place_prototype(),
        default_job_place_prototype(),
        generate_simple_demand(1..5),
        generate_no_skills(),
    )
}

pub fn default_job_prototype() -> impl Strategy<Value = JobVariant> {
    prop_oneof![default_delivery_prototype(), default_pickup_prototype(), default_pickup_delivery_prototype()]
}

pub fn default_costs_prototype() -> impl Strategy<Value = VehicleCosts> {
    from_costs(vec![
        VehicleCosts { fixed: Some(20.), distance: 0.0020, time: 0.003 },
        VehicleCosts { fixed: Some(30.), distance: 0.0015, time: 0.005 },
    ])
}

pub fn default_vehicle_location() -> Location {
    Location { lat: 0.0, lng: 0.0 }
}

pub fn default_vehicle_places_prototype() -> impl Strategy<Value = (VehiclePlace, Option<VehiclePlace>)> {
    let location = default_vehicle_location();
    Just((
        VehiclePlace { time: default_time_plus_offset(9), location: location.clone() },
        Some(VehiclePlace { time: default_time_plus_offset(18), location }),
    ))
}

pub fn default_breaks_prototype() -> impl Strategy<Value = Option<Vec<VehicleBreak>>> {
    Just(Some(vec![VehicleBreak {
        times: VehicleBreakTime::TimeWindows(vec![vec![default_time_plus_offset(12), default_time_plus_offset(14)]]),
        duration: 3600.,
        location: None,
    }]))
}

pub fn default_profiles() -> impl Strategy<Value = Vec<Profile>> {
    Just(vec![Profile { name: "car".to_string(), profile_type: "car".to_string() }])
}

pub fn default_vehicle_type_prototype() -> impl Strategy<Value = VehicleType> {
    generate_vehicle(
        from_ints(vec![2, 4]),
        Just("car".to_string()),
        generate_simple_capacity(30..50),
        default_costs_prototype(),
        generate_no_skills(),
        generate_no_limits(),
        generate_shifts(
            generate_shift(default_vehicle_places_prototype(), default_breaks_prototype(), generate_no_reloads()),
            1..2,
        ),
    )
}