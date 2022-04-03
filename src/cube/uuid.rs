use uuid::Uuid;

// UUIDs
const TOIO_UUID_SERVICE: Uuid = Uuid::from_u128(0x10B20100_5B3B_4571_9508_CF3EFCD7BBAE);
const TOIO_UUID_ID_INFO: Uuid = Uuid::from_u128(0x10B20101_5B3B_4571_9508_CF3EFCD7BBAE);
const TOIO_UUID_SENSOR_INFO: Uuid = Uuid::from_u128(0x10B20106_5B3B_4571_9508_CF3EFCD7BBAE);
const TOIO_UUID_BUTTON_INFO: Uuid = Uuid::from_u128(0x10B20107_5B3B_4571_9508_CF3EFCD7BBAE);
const TOIO_UUID_BATTERY_INFO: Uuid = Uuid::from_u128(0x10B20108_5B3B_4571_9508_CF3EFCD7BBAE);
const TOIO_UUID_MOTOR_CTRL: Uuid = Uuid::from_u128(0x10B20102_5B3B_4571_9508_CF3EFCD7BBAE);
const TOIO_UUID_LIGHT_CTRL: Uuid = Uuid::from_u128(0x10B20103_5B3B_4571_9508_CF3EFCD7BBAE);
const TOIO_UUID_SOUND_CTRL: Uuid = Uuid::from_u128(0x10B20104_5B3B_4571_9508_CF3EFCD7BBAE);
const TOIO_UUID_CONFIG: Uuid = Uuid::from_u128(0x10B201FF_5B3B_4571_9508_CF3EFCD7BBAE);

pub enum CoreCubeUuid {
    Service,
    IdInfo,
    SensorInfo,
    ButtonInfo,
    BatteryInfo,
    MotorCtrl,
    LightCtrl,
    SoundCtrl,
    Config,
}

impl CoreCubeUuid {
    pub fn uuid(&self) -> Uuid {
        match self {
            CoreCubeUuid::Service => TOIO_UUID_SERVICE,
            CoreCubeUuid::IdInfo => TOIO_UUID_ID_INFO,
            CoreCubeUuid::SensorInfo => TOIO_UUID_SENSOR_INFO,
            CoreCubeUuid::ButtonInfo => TOIO_UUID_BUTTON_INFO,
            CoreCubeUuid::BatteryInfo => TOIO_UUID_BATTERY_INFO,
            CoreCubeUuid::MotorCtrl => TOIO_UUID_MOTOR_CTRL,
            CoreCubeUuid::LightCtrl => TOIO_UUID_LIGHT_CTRL,
            CoreCubeUuid::SoundCtrl => TOIO_UUID_SOUND_CTRL,
            CoreCubeUuid::Config => TOIO_UUID_CONFIG,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            CoreCubeUuid::Service => "Service",
            CoreCubeUuid::IdInfo => "IdInfo",
            CoreCubeUuid::SensorInfo => "SensorInfo",
            CoreCubeUuid::ButtonInfo => "ButtonInfo",
            CoreCubeUuid::BatteryInfo => "BatteryInfo",
            CoreCubeUuid::MotorCtrl => "MotorCtrl",
            CoreCubeUuid::LightCtrl => "LightCtrl",
            CoreCubeUuid::SoundCtrl => "SoundCtrl",
            CoreCubeUuid::Config => "Config",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uuid_1() {
        let uuid_service = CoreCubeUuid::Service;
        println!("{}", CoreCubeUuid::Service.name());
        println!(
            "name: '{}', uuid: {:?}",
            uuid_service.name(),
            uuid_service.uuid()
        );
        assert_eq!("Service", uuid_service.name());
    }
}
