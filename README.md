<p align="center">
  <a href="">
    <img alt="npm version" src="https://badgen.net/github/commits/ahsanu123/wojo/">
  </a>
  <a href="">
    <img alt="npm" src="https://badgen.net/github/contributors/ahsanu123/wojo">
  </a>
  <a href="">
    <img alt="npm" src="https://badgen.net/github/branches/ahsanu123/wojo">
  </a>
  <a href="https://github.com/ahsanu123/wojo/blob/main/LICENSE">
    <img alt="licence" src="https://badgen.net/github/license/ahsanu123/wojo">
  </a>
</p>

![wojo_logo](wojo.svg) 

<p align="center">
  <b>
    🦷 Wojo - bluetooth client emulator app
  </b>
</p>


### Logs 

- 4 juli 2025, able to list all service and characteristic from bluetooth peripheral with `bluest`.
but to be able list all service and characteristic pc need to connect it first, then `bluest` able to read.

```shell


 getting connected devices
 found Device(DeviceImpl { inner: Device { adapter_name: hci0, address: FF:E4:05:1A:8F:FF }, session: Session { :1.69 } })
   Service(ServiceImpl { inner: Service { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, id: 1 }, device: Device { adapter_name: hci0, address: FF:E4:05:1A:8F:FF } })
     Characteristic(CharacteristicImpl { inner: Characteristic { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, service_id: 1, id: 2 } })
       props: CharacteristicProperties { broadcast: false, read: true, write_without_response: false, write: false, notify: false, indicate: false, authenticated_signed_writes: false, extended_properties: false, reliable_write: false, writable_auxiliaries: false }
       value: Ok([84, 114, 111, 117, 66, 76, 69])
     Characteristic(CharacteristicImpl { inner: Characteristic { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, service_id: 1, id: 4 } })
       props: CharacteristicProperties { broadcast: false, read: true, write_without_response: false, write: false, notify: false, indicate: false, authenticated_signed_writes: false, extended_properties: false, reliable_write: false, writable_auxiliaries: false }
       value: Ok([128, 7])
   Service(ServiceImpl { inner: Service { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, id: 32 }, device: Device { adapter_name: hci0, address: FF:E4:05:1A:8F:FF } })
     Characteristic(CharacteristicImpl { inner: Characteristic { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, service_id: 32, id: 38 } })
       props: CharacteristicProperties { broadcast: false, read: true, write_without_response: false, write: true, notify: true, indicate: false, authenticated_signed_writes: false, extended_properties: false, reliable_write: false, writable_auxiliaries: false }
       value: Ok([0])
       Descriptor(DescriptorImpl { inner: Descriptor { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, service_id: 32, characteristic_id: 38, id: 40 } }): Ok([0, 0])
     Characteristic(CharacteristicImpl { inner: Characteristic { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, service_id: 32, id: 33 } })
       props: CharacteristicProperties { broadcast: false, read: true, write_without_response: false, write: false, notify: true, indicate: false, authenticated_signed_writes: false, extended_properties: false, reliable_write: false, writable_auxiliaries: false }
       value: Ok([143])
       Descriptor(DescriptorImpl { inner: Descriptor { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, service_id: 32, characteristic_id: 33, id: 35 } }): Ok([1, 0])
       Descriptor(DescriptorImpl { inner: Descriptor { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, service_id: 32, characteristic_id: 33, id: 36 } }): Ok([0, 100])
       Descriptor(DescriptorImpl { inner: Descriptor { adapter_name: hci0, device_address: FF:E4:05:1A:8F:FF, service_id: 32, characteristic_id: 33, id: 37 } }): Ok([66, 97, 116, 116, 101, 114, 121, 32, 76, 101, 118, 101, 108])
 done

```

- another try using `btleplug`, its seem more robust (at least from the example)

```shell
$ sudo ./discover_adapters_peripherals

Starting scan on hci0 (usb:v1D6Bp0246d0552)...
Peripheral "Sudi Bluetooth" is connected: true
Now connected (true) to peripheral "Sudi Bluetooth"...
Discover peripheral "Sudi Bluetooth" services...
Service UUID 00001800-0000-1000-8000-00805f9b34fb, primary: true
  Characteristic { uuid: 00002a00-0000-1000-8000-00805f9b34fb, service_uuid: 00001800-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
  Characteristic { uuid: 00002a01-0000-1000-8000-00805f9b34fb, service_uuid: 00001800-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
Service UUID 0000180f-0000-1000-8000-00805f9b34fb, primary: true
  Characteristic { uuid: 00002a19-0000-1000-8000-00805f9b34fb, service_uuid: 0000180f-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ | NOTIFY), descriptors: {Descriptor { uuid: 00002902-0000-1000-8000-00805f9b34fb, service_uuid: 0000180f-0000-1000-8000-00805f9b34fb, characteristic_uuid: 00002a19-0000-1000-8000-00805f9b34fb }, Descriptor { uuid: 00002906-0000-1000-8000-00805f9b34fb, service_uuid: 0000180f-0000-1000-8000-00805f9b34fb, characteristic_uuid: 00002a19-0000-1000-8000-00805f9b34fb }, Descriptor { uuid: 00002912-0000-1000-8000-00805f9b34fb, service_uuid: 0000180f-0000-1000-8000-00805f9b34fb, characteristic_uuid: 00002a19-0000-1000-8000-00805f9b34fb }} }
  Characteristic { uuid: 408813df-5dd4-1f87-ec11-cdb001100000, service_uuid: 0000180f-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ | WRITE | NOTIFY), descriptors: {Descriptor { uuid: 00002902-0000-1000-8000-00805f9b34fb, service_uuid: 0000180f-0000-1000-8000-00805f9b34fb, characteristic_uuid: 408813df-5dd4-1f87-ec11-cdb001100000 }} }
Disconnecting from peripheral "Sudi Bluetooth"...
Peripheral "CB002" is connected: false
Connecting to peripheral "CB002"...
Error connecting to peripheral, skipping: br-connection-profile-unavailable

```

### 🎄 References

- https://github.com/deviceplug/btleplug
- https://github.com/ahsanu123/blendr
- https://github.com/alexmoon/bluest


