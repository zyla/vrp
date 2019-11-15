use core::construction::constraints::*;
use core::construction::states::{ActivityContext, RouteContext, SolutionContext};
use core::models::problem::TransportCost;
use std::slice::Iter;
use std::sync::Arc;

pub struct ReachableModule {
    constraints: Vec<ConstraintVariant>,
    keys: Vec<i32>,
}

impl ReachableModule {
    pub fn new(transport: Arc<dyn TransportCost + Send + Sync>, code: i32) -> Self {
        Self {
            constraints: vec![ConstraintVariant::HardActivity(Arc::new(ReachableHardActivityConstraint {
                transport,
                code,
            }))],
            keys: vec![],
        }
    }
}

impl ConstraintModule for ReachableModule {
    fn accept_route_state(&self, _ctx: &mut RouteContext) {}

    fn accept_solution_state(&self, _ctx: &mut SolutionContext) {}

    fn state_keys(&self) -> Iter<i32> {
        self.keys.iter()
    }

    fn get_constraints(&self) -> Iter<ConstraintVariant> {
        self.constraints.iter()
    }
}

struct ReachableHardActivityConstraint {
    transport: Arc<dyn TransportCost + Send + Sync>,
    code: i32,
}

impl HardActivityConstraint for ReachableHardActivityConstraint {
    fn evaluate_activity(
        &self,
        _route_ctx: &RouteContext,
        activity_ctx: &ActivityContext,
    ) -> Option<ActivityConstraintViolation> {
        let prev = activity_ctx.prev;
        let target = activity_ctx.target;
        let next = activity_ctx.next;

        let profile = _route_ctx.route.read().unwrap().actor.vehicle.profile;

        let prev_to_target =
            self.transport.distance(profile, prev.place.location, target.place.location, prev.schedule.departure);

        if prev_to_target < 0. {
            return Some(ActivityConstraintViolation { code: self.code, stopped: false });
        }

        if let Some(next) = next {
            let target_to_next =
                self.transport.distance(profile, target.place.location, next.place.location, target.schedule.departure);
            if target_to_next < 0. {
                return Some(ActivityConstraintViolation { code: self.code, stopped: false });
            }
        }

        None
    }
}