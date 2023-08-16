mod speed_information;
mod target_information;

use crate::characteristic::motor::def::CommandId;
use crate::payload::FromPayload;

/// Combined motor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MotorInformation {
    MotorControlTarget(target_information::ResponseMotorControlTarget),
    MotorControlMultipleTargets(target_information::ResponseMotorControlMultipleTargets),
    MotorSpeed(speed_information::MotorSpeedInformation),
}

impl FromPayload<&[u8]> for MotorInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.is_empty() {
            return None;
        }
        if let Some(response_data) =
            target_information::ResponseMotorControlTarget::from_payload(payload)
        {
            return Some(MotorInformation::MotorControlTarget(response_data));
        }
        if let Some(response_data) =
            target_information::ResponseMotorControlMultipleTargets::from_payload(payload)
        {
            return Some(MotorInformation::MotorControlMultipleTargets(response_data));
        }
        if let Some(response_data) = speed_information::MotorSpeedInformation::from_payload(payload)
        {
            return Some(MotorInformation::MotorSpeed(response_data));
        }
        None
    }
}

impl From<MotorInformation> for u8 {
    fn from(response_type: MotorInformation) -> u8 {
        match response_type {
            MotorInformation::MotorControlTarget(_) => CommandId::TargetPosition.response(),
            MotorInformation::MotorControlMultipleTargets(_) => {
                CommandId::MultiTargetPositions.response()
            }
            MotorInformation::MotorSpeed(_) => 0xe0u8,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::characteristic::motor::information::MotorInformation;
    use crate::characteristic::motor::def::ResponseCode;
    use crate::payload::FromPayload;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_response1() {
        _setup();

        let payload: [u8; 3] = [0xe0, 0x32, 0x00];
        let m = MotorInformation::from_payload(&payload);
        if let Some(MotorInformation::MotorSpeed(speed)) = m {
            println!("{:?}", speed);
            assert_eq!(speed.left, 0x32);
            assert_eq!(speed.right, 0x00);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn motor_response2() {
        _setup();

        let payload: [u8; 3] = [0x83, 0x00, 0x00];
        let m = MotorInformation::from_payload(&payload);
        if let Some(MotorInformation::MotorControlTarget(result)) = m {
            println!("{:?}", result);
            assert_eq!(result.response_code, ResponseCode::Success);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn motor_response3() {
        _setup();

        let payload: [u8; 3] = [0x84, 0x00, 0x00];
        let m = MotorInformation::from_payload(&payload);
        if let Some(MotorInformation::MotorControlMultipleTargets(result)) = m {
            println!("{:?}", result);
            assert_eq!(result.response_code, ResponseCode::Success);
        } else {
            unreachable!();
        }
    }
}
