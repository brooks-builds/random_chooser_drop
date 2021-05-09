use nalgebra::Isometry2;
use rapier2d::dynamics::{
    CCDSolver, IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};
use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase};
use rapier2d::pipeline::PhysicsPipeline;

use crate::config::config_struct::Config;
use crate::event_manager::EventManager;
use crate::helpers::vector2::Vector2;

use self::event_handler::PhysicsEventHandler;

pub mod event_handler;

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
    event_handler: PhysicsEventHandler,
}

impl Physics {
    pub fn new(config: &Config, event_manager: &mut EventManager) -> Self {
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
            event_handler: PhysicsEventHandler::new(event_manager),
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

    pub fn insert_rotated_wall(
        &mut self,
        position: Vector2,
        width: f32,
        height: f32,
        rotation: f32,
        height_offset: f32,
    ) -> u128 {
        let id = self.last_used_id + 1;
        let position = Isometry2::new(position.to_nalgebra(), 0.0);
        let wall = RigidBodyBuilder::new_static()
            .position(position)
            .user_data(id)
            .build();
        let handle = self.bodies.insert(wall);
        let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0)
            .translation(0.0, -(height / 2.0 + height_offset))
            .rotation(rotation)
            .user_data(id)
            .build();
        self.colliders.insert(collider, handle, &mut self.bodies);

        self.last_used_id = id;
        id
    }

    pub fn insert_nail(&mut self, position: Vector2, radius: f32) -> u128 {
        let id = self.last_used_id + 1;
        self.last_used_id = id;
        let nail = RigidBodyBuilder::new_static()
            .position(Isometry2::new(position.to_nalgebra(), 0.0))
            .user_data(id)
            .build();
        let handle = self.bodies.insert(nail);
        let collider = ColliderBuilder::ball(radius).user_data(id).build();
        self.colliders.insert(collider, handle, &mut self.bodies);

        id
    }

    pub fn insert_sensor(&mut self, position: Vector2, width: f32, height: f32) -> u128 {
        let id = self.last_used_id + 1;
        let body = RigidBodyBuilder::new_static()
            .position(Isometry2::new(position.to_nalgebra(), 0.0))
            .user_data(id)
            .build();
        let handle = self.bodies.insert(body);
        let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0)
            .sensor(true)
            .user_data(id)
            .build();

        self.colliders.insert(collider, handle, &mut self.bodies);

        self.last_used_id += 1;
        id
    }

    pub fn update(&mut self) {
        let hooks = ();

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
            &self.event_handler,
        );
    }

    pub fn remove(&mut self, handle: RigidBodyHandle) {
        self.bodies
            .remove(handle, &mut self.colliders, &mut self.joints);
    }

    pub fn get_rigid_body_handle(&self, id: u128) -> Option<RigidBodyHandle> {
        for (handle, body) in self.bodies.iter() {
            if body.user_data == id {
                return Some(handle);
            }
        }

        None
    }
}
