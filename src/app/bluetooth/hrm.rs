use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/ble_devices.js")]
extern "C" {
    fn name() -> String;

    type HRMDevice;
    #[wasm_bindgen(constructor)]
    fn new(on_heartrate: &Closure<dyn FnMut(&JsValue)>, on_state_change: &Closure<dyn FnMut(&JsValue)>) -> HRMDevice;

    #[wasm_bindgen(method)]
    fn connect(this: &HRMDevice);
}


pub struct HRM {
    on_heartrate: Closure<dyn FnMut(&JsValue)>,
    on_state_change: Closure<dyn FnMut(&JsValue)>,
    hrm: HRMDevice,
}

impl HRM {
    pub fn new<F: 'static, G: 'static>(on_hr : F, on_state: G) -> HRM
    where F: FnMut(&JsValue), G: FnMut(&JsValue) {
        let on_heartrate = Closure::new(on_hr);
        let on_state_change = Closure::new(on_state);
        let hrm = HRMDevice::new(&on_heartrate, &on_state_change);
        HRM {
            on_heartrate,
            on_state_change,
            hrm,
        }
    }

    pub fn reconnect_hrm(&self) {
        self.hrm.connect();
    }
}

