// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2026 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

use crate::device::{
  hardware::{HardwareCommand, HardwareWriteCmd},
  protocol::{ProtocolHandler, generic_protocol_setup},
};
use buttplug_core::errors::ButtplugDeviceError;
use buttplug_server_device_config::Endpoint;
use uuid::{Uuid, uuid};

const KIIROO_POWERBLOW_PROTOCOL_UUID: Uuid = uuid!("93458b1f-4ed5-4744-9c88-e30ab46c601f");

generic_protocol_setup!(KiirooPowerBlow, "kiiroo-powerblow");

#[derive(Default)]
pub struct KiirooPowerBlow {}

impl ProtocolHandler for KiirooPowerBlow {
  // PowerBlow is a suction/vacuum pump, exposed as a single Constrict actuator.
  //   level 1..=255 -> build vacuum: write [level, speed] to suction char (Endpoint::Tx     = 0x1401)
  //   level 0        -> stop+release: zero the suction char, then pulse release char (Endpoint::TxMode = 0x1402)
  // Packet is 2 bytes [intensity, speed], each 0-255 (byteEncoding, template "pos;speed").
  // speed 0xff = fastest pump; lower it for a gentler ramp.
  fn handle_output_constrict_cmd(
    &self,
    _feature_index: u32,
    _feature_id: Uuid,
    level: u32,
  ) -> Result<Vec<HardwareCommand>, ButtplugDeviceError> {
    let level = level.min(255) as u8;
    const SPEED: u8 = 0xff;
    if level == 0 {
      // Zero the suction command AND trigger release, otherwise the suction
      // characteristic stays latched at its last value.
      Ok(vec![
        HardwareWriteCmd::new(&[KIIROO_POWERBLOW_PROTOCOL_UUID], Endpoint::Tx, vec![0x00, SPEED], true).into(),
        HardwareWriteCmd::new(&[KIIROO_POWERBLOW_PROTOCOL_UUID], Endpoint::TxMode, vec![0xff, SPEED], true).into(),
      ])
    } else {
      Ok(vec![
        HardwareWriteCmd::new(&[KIIROO_POWERBLOW_PROTOCOL_UUID], Endpoint::Tx, vec![level, SPEED], true).into(),
      ])
    }
  }
}
