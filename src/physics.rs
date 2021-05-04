use nalgebra::Isometry2;
use rapier2d::dynamics::{
    CCDSolver, IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodySet,
};
use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase};
use rapier2d::pipeline::PhysicsPipeline;

use crate::config::config_struct::Config;
use crate::helpers::vector2::Vector2;

pub struct Physics {
    pipeline: PhysicsPipeline,
    gravity: Vector2,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
    last_used_id: u128,
}

impl Physics {
    pub fn new(config: &Config) -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: Vector2::new(0.0, config.gravity),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            last_used_id: 0,
        }
    }

    pub fn insert_ball(&mut self, position: Vector2, radius: f32, restitution: f32) -> u128 {
        let id = self.last_used_id + 1;
        let ball = RigidBodyBuilder::new_dynamic()
            .position(Isometry2::new(position.to_nalgebra(), 0.0))
            .user_data(id)
            .build();
        let body_handle = self.bodies.insert(ball);
        let collider = ColliderBuilder::ball(radius)
            .restitution(restitution)
            .user_data(id)
            .build();
        self.colliders
            .insert(collider, body_handle, &mut self.bodies);

        self.last_used_id = id;
        id
    }

    pub fn insert_wall(&mut self, position: Vector2, width: f32, height: f32) -> u128 {
        let id = self.last_used_id + 1;
        let position = Isometry2::new(position.to_nalgebra(), 0.0);
        let wall = RigidBodyBuilder::new_static()
            .position(position)
            .user_data(id)
            .build();
        let handle = self.bodies.insert(wall);
        let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0)
            .user_data(id)
            .build();
        self.colliders.insert(collider, handle, &mut self.bodies);

        self.last_used_id = id;
        id
    }

    pub fn update(&mut self) {
        let hooks = ();
        let event_handlers = ();
        self.pipeline.step(
            self.gravity.get_nalgebra(),
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            &hooks,
            &event_handlers,
        );
    }
}
