//! An introduction to the usage of RuBullet.
use std::{thread, time::Duration};
use nidhogg::{Result, NaoBackend};
use nidhogg::backend::BulletBackend;

fn main() -> Result<()> {
    let mut bullet = BulletBackend::connect()?;

    for _ in 0..10000 {
        bullet.physics_client.step_simulation()?;
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
