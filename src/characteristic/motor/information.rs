mod speed_information;
mod target_information;

use crate::characteristic::motor::def::CommandId;
use crate::payload::ToPayload;

/// Combined motor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MotorInformation {
    MotorControlTarget(target_information::ResponseMotorControlTarget),
    MotorControlMultipleTargets(target_information::ResponseMotorControlMultipleTargets),
    MotorSpeed(speed_information::MotorSpeedInformation),
}

impl MotorInformation {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.is_empty() {
            return None;
        }
        if let Some(response_data) = target_information::ResponseMotorControlTarget::new(byte_data)
        {
            return Some(MotorInformation::MotorControlTarget(response_data));
        }
        if let Some(response_data) =
            target_information::ResponseMotorControlMultipleTargets::new(byte_data)
        {
            return Some(MotorInformation::MotorControlMultipleTargets(response_data));
        }
        if let Some(response_data) = speed_information::MotorSpeedInformation::new(byte_data) {
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

impl ToPayload<Vec<u8>> for MotorInformation {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            MotorInformation::MotorControlTarget(st) => {
                payload.extend(st.to_payload());
            }
            MotorInformation::MotorControlMultipleTargets(st) => {
                payload.extend(st.to_payload());
            }
            MotorInformation::MotorSpeed(st) => {
                payload.extend(st.to_payload());
            }
        }
        payload
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::characteristic::motor::def::RequestId;
    use crate::characteristic::motor::def::ResponseCode;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_response1() {
        _setup();

        let res = MotorInformation::MotorSpeed(speed_information::MotorSpeedInformation {
            left: 10,
            right: 11,
        });
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 3);
    }

    #[test]
    fn motor_response2() {
        _setup();

        let res =
            MotorInformation::MotorControlTarget(target_information::ResponseMotorControlTarget {
                request_id: RequestId::new(),
                response_code: ResponseCode::ErrorTimeout,
            });
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 3);
    }

    #[test]
    fn motor_response3() {
        _setup();

        let res = MotorInformation::MotorControlMultipleTargets(
            target_information::ResponseMotorControlMultipleTargets {
                request_id: RequestId::new(),
                response_code: ResponseCode::ErrorInvalidParameter,
            },
        );
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 3);
    }
}
