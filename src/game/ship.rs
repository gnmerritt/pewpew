use na::{Vector3, Rotation3, Translation3};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Ship {
    position: Vector3<f32>,
    orientation: Rotation3<f32>,
    velocity: Vector3<f32>,
}

impl Ship {
    pub fn at_origin() -> Ship {
        Ship {
            position: Vector3::new(0.0, 0.0, 0.0),
            orientation: Rotation3::identity(),
            velocity: Vector3::new(1.0, 0.0, 0.0),
        }
    }

    pub fn translation(&self) -> Translation3<f32> {
        let pos = self.position;
        Translation3::new(pos.x, pos.y, pos.z)
    }
}

#[cfg(test)]
mod test {
    use na::{Vector3, Rotation3};
    use bincode::{serialize, deserialize, Infinite};
    use super::Ship;

    #[test]
    fn serialization() {
        let ship = Ship {
            position: Vector3::new(0.0, 1.0, 0.0),
            orientation: Rotation3::identity(),
            velocity: Vector3::new(1.0, 0.0, 0.0),
        };
        let encoded: Vec<u8> = serialize(&ship, Infinite).unwrap();

        assert_eq!(encoded.len(), 60); // TODO: why is this 60 bytes?

        let decoded: Ship = deserialize(&encoded[..]).unwrap();

        assert_eq!(ship, decoded);
    }
}
