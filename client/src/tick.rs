// Library
use coord::prelude::*;

// Project
use region::{physics, VolState};

// Local
use Client;
use ClientStatus;
use Payloads;
use CHUNK_SIZE;

impl<P: Payloads> Client<P> {
    pub(crate) fn tick(&self, dt: f32) -> bool {
        self.update_chunks();
        let entities = self.entities.read().unwrap();

        // Physics tick
        {
            // Take the physics lock to sync client and frontend updates
            let _ = self.take_phys_lock();
            physics::tick(entities.iter(), &self.chunk_mgr, CHUNK_SIZE, dt);
        }

        self.update_server();

        *self.time.write().unwrap() += dt as f64;

        *self.status() != ClientStatus::Disconnected
    }
}
