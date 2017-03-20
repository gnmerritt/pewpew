use na::{Vector3, Rotation3};

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
}

#[cfg(test)]
mod test {
    use na::{Vector3, Rotation3};
    use bincode::{serialize, deserialize, SizeLimit};
    use super::Ship;

    #[test]
    fn serialization() {
        let ship = Ship {
            position: Vector3::new(0.0, 1.0, 0.0),
            orientation: Rotation3::identity(),
            velocity: Vector3::new(1.0, 0.0, 0.0),
        };
        let encoded: Vec<u8> = serialize(&ship, SizeLimit::Infinite).unwrap();

        assert_eq!(encoded.len(), 60); // TODO: why is this 60 bytes?

        let decoded: Ship = deserialize(&encoded[..]).unwrap();

        assert_eq!(ship, decoded);
    }
}
