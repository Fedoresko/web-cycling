let BLE_ATTRIBUTES = {
    "0000180d-0000-1000-8000-00805f9b34fb": "Heart Rate Service",
    "0000180a-0000-1000-8000-00805f9b34fb": "Device Information Service",
            // Sample Characteristics.
    "00002a37-0000-1000-8000-00805f9b34fb": "Heart Rate Measurement",
    "00002a29-0000-1000-8000-00805f9b34fb": "Manufacturer Name String",
  
            // GATT Services
    "00001800-0000-1000-8000-00805f9b34fb": "Generic Access",
    "00001801-0000-1000-8000-00805f9b34fb": "Generic Attribute",
  
  
            // GATT Declarations
    "00002800-0000-1000-8000-00805f9b34fb": "Primary Service",
    "00002801-0000-1000-8000-00805f9b34fb": "Secondary Service",
    "00002802-0000-1000-8000-00805f9b34fb": "Include",
    "00002803-0000-1000-8000-00805f9b34fb": "Characteristic",
  
            // GATT Descriptors
    "00002900-0000-1000-8000-00805f9b34fb": "Characteristic Extended Properties",
    "00002901-0000-1000-8000-00805f9b34fb": "Characteristic User Description",
    "00002902-0000-1000-8000-00805f9b34fb": "Client Characteristic Configuration",
    "00002903-0000-1000-8000-00805f9b34fb": "Server Characteristic Configuration",
    "00002904-0000-1000-8000-00805f9b34fb": "Characteristic Presentation Format",
    "00002905-0000-1000-8000-00805f9b34fb": "Characteristic Aggregate Format",
    "00002906-0000-1000-8000-00805f9b34fb": "Valid Range",
    "00002907-0000-1000-8000-00805f9b34fb": "External Report Reference Descriptor",
    "00002908-0000-1000-8000-00805f9b34fb": "Report Reference Descriptor",
  
            // GATT Characteristics
    "00002a00-0000-1000-8000-00805f9b34fb": "Device Name",
    "00002a01-0000-1000-8000-00805f9b34fb": "Appearance",
    "00002a02-0000-1000-8000-00805f9b34fb": "Peripheral Privacy Flag",
    "00002a03-0000-1000-8000-00805f9b34fb": "Reconnection Address",
    "00002a04-0000-1000-8000-00805f9b34fb": "PPCP",
    "00002a05-0000-1000-8000-00805f9b34fb": "Service Changed",
  
            // GATT Service UUIDs
    "00001802-0000-1000-8000-00805f9b34fb": "Immediate Alert",
    "00001803-0000-1000-8000-00805f9b34fb": "Link Loss",
    "00001804-0000-1000-8000-00805f9b34fb": "Tx Power",
    "00001805-0000-1000-8000-00805f9b34fb": "Current Time Service",
    "00001806-0000-1000-8000-00805f9b34fb": "Reference Time Update Service",
    "00001807-0000-1000-8000-00805f9b34fb": "Next DST Change Service",
    "00001808-0000-1000-8000-00805f9b34fb": "Glucose",
    "00001809-0000-1000-8000-00805f9b34fb": "Health Thermometer",
    "0000180a-0000-1000-8000-00805f9b34fb": "Device Information",
    "0000180b-0000-1000-8000-00805f9b34fb": "Network Availability",
    "0000180d-0000-1000-8000-00805f9b34fb": "Heart Rate",
    "0000180e-0000-1000-8000-00805f9b34fb": "Phone Alert Status Service",
    "0000180f-0000-1000-8000-00805f9b34fb": "Battery Service",
    "00001810-0000-1000-8000-00805f9b34fb": "Blood Pressure",
    "00001811-0000-1000-8000-00805f9b34fb": "Alert Notification Service",
    "00001812-0000-1000-8000-00805f9b34fb": "Human Interface Device",
    "00001813-0000-1000-8000-00805f9b34fb": "Scan Parameters",
    "00001814-0000-1000-8000-00805f9b34fb": "Running Speed and Cadence",
    "00001816-0000-1000-8000-00805f9b34fb": "Cycling Speed and Cadence",
    "00001818-0000-1000-8000-00805f9b34fb": "Cycling Power",
    "00001819-0000-1000-8000-00805f9b34fb": "Location and Navigation",
  
            // GATT Characteristic UUIDs
    "00002a06-0000-1000-8000-00805f9b34fb": "Alert Level",
    "00002a07-0000-1000-8000-00805f9b34fb": "Tx Power Level",
    "00002a08-0000-1000-8000-00805f9b34fb": "Date Time",
    "00002a09-0000-1000-8000-00805f9b34fb": "Day of Week",
    "00002a0a-0000-1000-8000-00805f9b34fb": "Day Date Time",
    "00002a0c-0000-1000-8000-00805f9b34fb": "Exact Time 256",
    "00002a0d-0000-1000-8000-00805f9b34fb": "DST Offset",
    "00002a0e-0000-1000-8000-00805f9b34fb": "Time Zone",
    "00002a0f-0000-1000-8000-00805f9b34fb": "Local Time Information",
    "00002a11-0000-1000-8000-00805f9b34fb": "Time with DST",
    "00002a12-0000-1000-8000-00805f9b34fb": "Time Accuracy",
    "00002a13-0000-1000-8000-00805f9b34fb": "Time Source",
    "00002a14-0000-1000-8000-00805f9b34fb": "Reference Time Information",
    "00002a16-0000-1000-8000-00805f9b34fb": "Time Update Control Point",
    "00002a17-0000-1000-8000-00805f9b34fb": "Time Update State",
    "00002a18-0000-1000-8000-00805f9b34fb": "Glucose Measurement",
    "00002a19-0000-1000-8000-00805f9b34fb": "Battery Level",
    "00002a1c-0000-1000-8000-00805f9b34fb": "Temperature Measurement",
    "00002a1d-0000-1000-8000-00805f9b34fb": "Temperature Type",
    "00002a1e-0000-1000-8000-00805f9b34fb": "Intermediate Temperature",
    "00002a21-0000-1000-8000-00805f9b34fb": "Measurement Interval",
    "00002a22-0000-1000-8000-00805f9b34fb": "Boot Keyboard Input Report",
    "00002a23-0000-1000-8000-00805f9b34fb": "System ID",
    "00002a24-0000-1000-8000-00805f9b34fb": "Model Number String",
    "00002a25-0000-1000-8000-00805f9b34fb": "Serial Number String",
    "00002a26-0000-1000-8000-00805f9b34fb": "Firmware Revision String",
    "00002a27-0000-1000-8000-00805f9b34fb": "Hardware Revision String",
    "00002a28-0000-1000-8000-00805f9b34fb": "Software Revision String",
    "00002a29-0000-1000-8000-00805f9b34fb": "Manufacturer Name String",
    "00002a2a-0000-1000-8000-00805f9b34fb": "IEEE 11073-20601 Regulatory Certification Data List",
    "00002a2b-0000-1000-8000-00805f9b34fb": "Current Time",
    "00002a31-0000-1000-8000-00805f9b34fb": "Scan Refresh",
    "00002a32-0000-1000-8000-00805f9b34fb": "Boot Keyboard Output Report",
    "00002a33-0000-1000-8000-00805f9b34fb": "Boot Mouse Input Report",
    "00002a34-0000-1000-8000-00805f9b34fb": "Glucose Measurement Context",
    "00002a35-0000-1000-8000-00805f9b34fb": "Blood Pressure Measurement",
    "00002a36-0000-1000-8000-00805f9b34fb": "Intermediate Cuff Pressure",
    "00002a37-0000-1000-8000-00805f9b34fb": "Heart Rate Measurement",
    "00002a38-0000-1000-8000-00805f9b34fb": "Body Sensor Location",
    "00002a39-0000-1000-8000-00805f9b34fb": "Heart Rate Control Point",
    "00002a3e-0000-1000-8000-00805f9b34fb": "Network Availability",
    "00002a3f-0000-1000-8000-00805f9b34fb": "Alert Status",
    "00002a40-0000-1000-8000-00805f9b34fb": "Ringer Control Point",
    "00002a41-0000-1000-8000-00805f9b34fb": "Ringer Setting",
    "00002a42-0000-1000-8000-00805f9b34fb": "Alert Category ID Bit Mask",
    "00002a43-0000-1000-8000-00805f9b34fb": "Alert Category ID",
    "00002a44-0000-1000-8000-00805f9b34fb": "Alert Notification Control Point",
    "00002a45-0000-1000-8000-00805f9b34fb": "Unread Alert Status",
    "00002a46-0000-1000-8000-00805f9b34fb": "New Alert",
    "00002a47-0000-1000-8000-00805f9b34fb": "Supported New Alert Category",
    "00002a48-0000-1000-8000-00805f9b34fb": "Supported Unread Alert Category",
    "00002a49-0000-1000-8000-00805f9b34fb": "Blood Pressure Feature",
    "00002a4a-0000-1000-8000-00805f9b34fb": "HID Information",
    "00002a4b-0000-1000-8000-00805f9b34fb": "Report Map",
    "00002a4c-0000-1000-8000-00805f9b34fb": "HID Control Point",
    "00002a4d-0000-1000-8000-00805f9b34fb": "Report",
    "00002a4e-0000-1000-8000-00805f9b34fb": "Protocol Mode",
    "00002a4f-0000-1000-8000-00805f9b34fb": "Scan Interval Window",
    "00002a50-0000-1000-8000-00805f9b34fb": "PnP ID",
    "00002a51-0000-1000-8000-00805f9b34fb": "Glucose Feature",
    "00002a52-0000-1000-8000-00805f9b34fb": "Record Access Control Point",
    "00002a53-0000-1000-8000-00805f9b34fb": "RSC Measurement",
    "00002a54-0000-1000-8000-00805f9b34fb": "RSC Feature",
    "00002a55-0000-1000-8000-00805f9b34fb": "SC Control Point",
    "00002a5b-0000-1000-8000-00805f9b34fb": "CSC Measurement",
    "00002a5c-0000-1000-8000-00805f9b34fb": "CSC Feature",
    "00002a5d-0000-1000-8000-00805f9b34fb": "Sensor Location",
    "00002a63-0000-1000-8000-00805f9b34fb": "Cycling Power Measurement",
    "00002a64-0000-1000-8000-00805f9b34fb": "Cycling Power Vector",
    "00002a65-0000-1000-8000-00805f9b34fb": "Cycling Power Feature",
    "00002a66-0000-1000-8000-00805f9b34fb": "Cycling Power Control Point",
    "00002a67-0000-1000-8000-00805f9b34fb": "Location and Speed",
    "00002a68-0000-1000-8000-00805f9b34fb": "Navigation",
    "00002a69-0000-1000-8000-00805f9b34fb": "Position Quality",
    "00002a6a-0000-1000-8000-00805f9b34fb": "LN Feature",
    "00002a6b-0000-1000-8000-00805f9b34fb": "LN Control Point"}

class BLEDevice {
    #primaryServices;
    #optionalServices;
    #chosenService = null;
    #intervalId = 0;
    sensorState = {};

    constructor(primaryServices, optionalServices)  {
      this.#primaryServices = primaryServices;
      this.#optionalServices = optionalServices;
    }

    connect() {
      navigator.bluetooth.requestDevice({
        filters: [{
          services: Object.keys(this.#primaryServices)
        }],
        optionalServices: Object.keys(this.#optionalServices)
      }).then(device => {
        device.gatt.connect().then(this.subDevice.bind(this));
        device.addEventListener('gattserverdisconnected', this.onDisconnect.bind(this));
      });

      if (this.#intervalId != 0) {
        clearInterval(this.#intervalId);
        this.#intervalId = 0;
      }

      this.#intervalId = setInterval(() => {
        if (this.#chosenService != null) {
          let connected = this.#chosenService.device.gatt.connected;

          console.log("Connected: " + connected);
          if (!connected) {
            this.#chosenService.device.gatt.connect().then(this.subDevice.bind(this));
          }
        }
      }, 3000);

    }

    onDisconnect() {
      console.log("Disconnect!");
      this.#chosenService.device.gatt.connect().then(this.subDevice.bind(this));
    }

    subDevice(server) {
      this.sensorState.name = server.device.name;

      Object.entries(this.#primaryServices).forEach( (entry) => {
        const [name, characteristics] = entry;
        server.getPrimaryService(name).then(service => {
          this.#chosenService = service;
          return Promise.all( Object.entries(characteristics).map( (char) => {
              service.getCharacteristic(char[0]).then(char[1]).catch(error => {
                console.log('Cant add feature '+char[0]+" for service "+name+" "+error);
              } )
            }
          ))
        }).catch(error => {console.log('Cant add service '+name+" "+error)})
      });

      Object.entries(this.#optionalServices).forEach( (entry) => {
        const [name, characteristics] = entry;
        server.getPrimaryService(name).then(service => {
          this.#chosenService = service;
          return Promise.all( Object.entries(characteristics).map( (char) => {
              service.getCharacteristic(char[0]).then(char[1]).catch(error => {
                console.log('Cant add feature '+char[0]+" for service "+name+" "+error);
              } )
            }
          ))
        }).catch(error => {console.log('Cant add service '+name+" "+error)})
      });
    }
}

export class HRMDevice {
    #onHeartRate;
    #onStateChange;
    #device;

    constructor(onHeartRate, onStateChange)  {
      this.#device = new BLEDevice( {
        'heart_rate' : {
          'body_sensor_location' : this.handleBodySensorLocationCharacteristic.bind(this),
          'heart_rate_measurement' : this.handleHeartRateMeasurementCharacteristic.bind(this),
        }
      }, {
        'battery_service' : {
          'battery_level' : this.handleBatteryLevel.bind(this)
        },
        'device_information' : c => {}
      } );
      this.#onHeartRate = onHeartRate;
      this.#onStateChange = onStateChange;
    }

    connect() {
      this.#device.connect();
    }

    handleBodySensorLocationCharacteristic(characteristic) {
      if (characteristic === null) {
        console.log("Unknown sensor location.");
        return Promise.resolve();
      }
      return characteristic.readValue()
      .then(sensorLocationData => {
        const sensorLocation = sensorLocationData.getUint8(0);
        switch (sensorLocation) {
          case 0: return 'Other';
          case 1: return 'Chest';
          case 2: return 'Wrist';
          case 3: return 'Finger';
          case 4: return 'Hand';
          case 5: return 'Ear Lobe';
          case 6: return 'Foot';
          default: return 'Unknown';
        }
      }).then(location => {
        this.#device.sensorState.location = location;
        this.#onStateChange(this.#device.sensorState);
      });
    }

    handleBatteryLevel(characteristic) {
      if (characteristic === null) {
        console.log("Unknown sensor location.");
        return Promise.resolve();
      }
      return characteristic.readValue()
      .then(batLev => {
        this.#device.sensorState.battery = batLev;
        this.#onStateChange(this.#device.sensorState);
      });
    }

    handleDeviceInfo(characteristics) {
      characteristics.forEach(characteristic => {
        let uuid = characteristic.uuid;
        characteristic.readValue()
        .then(deviceInfo => {
          let decoder = new TextDecoder('windows-1251');
          let decodedString = decoder.decode(deviceInfo);
          this.#device.sensorState.deviceInfo = decodedString;
          console.log(BLE_ATTRIBUTES[uuid] + ": " + decodedString);
        });
      });
    }

    handleHeartRateMeasurementCharacteristic(characteristic) {
      return characteristic.startNotifications()
      .then(char => {
        characteristic.addEventListener('characteristicvaluechanged',
                                        this.parseHeartRate.bind(this));
      });
    }

    parseHeartRate(event) {
      const characteristic = event.target;
      let data = characteristic.value;

      const flags = data.getUint8(0);
      const rate16Bits = flags & 0x1;
      const result = {};
      let index = 1;
      if (rate16Bits) {
        result.heartRate = data.getUint16(index, /*littleEndian=*/true);
        index += 2;
      } else {
        result.heartRate = data.getUint8(index);
        index += 1;
      }
      const contactDetected = flags & 0x2;
      const contactSensorPresent = flags & 0x4;
      if (contactSensorPresent) {
        result.contactDetected = !!contactDetected;
      }
      const energyPresent = flags & 0x8;
      if (energyPresent) {
        result.energyExpended = data.getUint16(index, /*littleEndian=*/true);
        index += 2;
      }
      const rrIntervalPresent = flags & 0x10;
      if (rrIntervalPresent) {
        const rrIntervals = [];
        for (; index + 1 < data.byteLength; index += 2) {
          rrIntervals.push(data.getUint16(index, /*littleEndian=*/true));
        }
        result.rrIntervals = rrIntervals;
      }
      this.#onHeartRate(result);
    }
}

export class PowerTrainer {
    #onPower;
    #onStateChange;
    #device;

    constructor(onPower, onStateChange)  {
      this.#device = new BLEDevice( {
        'cycling_power' : {
          'cycling_power_measurement' : this.subscribeForPowerMeasure(),
          'cycling_power_feature' : this.logCharacteristic.bind(this),
          'cycling_power_vector' : this.subscribeForLog()
        }
      }, {
        'cycling_speed_and_cadence' : {
          'csc_measurement' : this.subscribeForLog(),
          'csc_feature' : this.logCharacteristic.bind(this)
        },
        'fitness_machine' : {
          'fitness_machine_feature' : this.logCharacteristic.bind(this),
          'indoor_bike_data': this.subscribeForIndoorBikeCharcateristic(),
          'supported_power_range': this.logCharacteristic.bind(this)
        },
        'device_information' : c => {}
      } );
      this.#onPower = onPower;
      this.#onStateChange = onStateChange;
    }

    connect() {
      this.#device.connect();
    }

    logCharacteristic(characteristic) {
      characteristic.readValue().then( val => {
        console.log(BLE_ATTRIBUTES[characteristic.uuid]+ ": "+val.getUint16());
      }).catch(e => console.log("Cant get value for "+BLE_ATTRIBUTES[characteristic.uuid]+" "+e) );
    }

    subscribeCharacteristic(acceptor) {
      return characteristic => { characteristic.startNotifications()
        .then(char => {
            characteristic.addEventListener('characteristicvaluechanged', acceptor);
        }).catch(e => console.log("Cant subscribe for "+BLE_ATTRIBUTES[characteristic.uuid]+" "+e) )
      }
    }

    subscribeForLog() {
      return this.subscribeCharacteristic(event => {
          console.log(BLE_ATTRIBUTES[event.target.uuid] + ": " + event.target.value);
        });
    }

    bit_test(num, bit){
      return ((num>>bit) % 2 != 0)
    }

    subscribeForIndoorBikeCharcateristic() {
      return this.subscribeCharacteristic(event => {
        let flags = event.target.value.getUint16(0, true);
        let pos = 2;
  //      if (this.bit_test(flags, 0)) {
        this.#device.sensorState.speed = event.target.value.getInt16(pos, true);
        pos += 2;
  //      }
        if (this.bit_test(flags, 1)) {
          this.#device.sensorState.averageSpeed = event.target.value.getUint16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 2)) {
          this.#device.sensorState.cadence = event.target.value.getUint16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 3)) {
          this.#device.sensorState.averageCadence = event.target.value.getUint16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 4)) {
          this.#device.sensorState.totalDistance = event.target.value.getUint16(pos, true);
          pos += 2;
          this.#device.sensorState.totalDistance2 = event.target.value.getUint8(pos, true);
          pos += 1;
        }
        if (this.bit_test(flags, 5)) {
          this.#device.sensorState.resistanceLevel = event.target.value.getInt16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 6)) {
          this.#device.sensorState.instPower = event.target.value.getInt16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 7)) {
          this.#device.sensorState.averagePower = event.target.value.getInt16(pos, true);
          pos += 2;
        }

        console.log("State :"+JSON.stringify(this.#device.sensorState));
      });
    }

    subscribeForPowerMeasure() {
      return this.subscribeCharacteristic(event => {
        let flags = event.target.value.getUint16(0, true);
        let pos = 2;
        this.#device.sensorState.power = event.target.value.getInt16(pos, true);
        pos += 2;
        if (this.bit_test(flags, 0)) {
          this.#device.sensorState.powerBalance = event.target.value.getUint8(pos, true);
          pos += 1;
        }
        if (this.bit_test(flags, 2)) {
          this.#device.sensorState.accumulatedTorque = event.target.value.getUint16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 4)) {
          this.#device.sensorState.wheelRevolutions = event.target.value.getUint32(pos, true);
          pos += 4;
          this.#device.sensorState.lastWheelRevolution = event.target.value.getUint16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 5)) {
          this.#device.sensorState.crankRevolutions = event.target.value.getUint16(pos, true);
          pos += 2;
          this.#device.sensorState.lastCrankRevolution = event.target.value.getUint16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 6)) {
          this.#device.sensorState.maxForce = event.target.value.getInt16(pos, true);
          pos += 2;
          this.#device.sensorState.minForce = event.target.value.getInt16(pos, true);
          pos += 2;
        }
        if (this.bit_test(flags, 7)) {
          this.#device.sensorState.maxTorque = event.target.value.getInt16(pos, true);
          pos += 2;
          this.#device.sensorState.minTorque = event.target.value.getInt16(pos, true);
          pos += 2;
        }

        console.log("State :"+JSON.stringify(this.#device.sensorState));
      });
    }
}