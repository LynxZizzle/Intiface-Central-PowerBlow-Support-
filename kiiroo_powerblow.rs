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
  // PowerBlow suction is a held level. Packet = 2 bytes [strength, speed]:
  //   byte 0 (strength): 0x01 = off ... 0xff = max, linear. Held (no re-pump needed).
  //   byte 1 (speed):    0xff = fastest.
  // Exposed as a single Constrict actuator. Config value range is [1, 255] so a
  // client's 100% maps to strength 0xff. level 0 = release.
  fn handle_output_constrict_cmd(
    &self,
    _feature_index: u32,
    _feature_id: Uuid,
    level: u32,
  ) -> Result<Vec<HardwareCommand>, ButtplugDeviceError> {
    let level = level.min(255) as u8;
    if level == 0 {
      // Release: zero the suction char, then pulse the release char (0x1402).
      Ok(vec![
        HardwareWriteCmd::new(&[KIIROO_POWERBLOW_PROTOCOL_UUID], Endpoint::Tx, vec![0x00, 0xff], true).into(),
        HardwareWriteCmd::new(&[KIIROO_POWERBLOW_PROTOCOL_UUID], Endpoint::TxMode, vec![0xff, 0xff], true).into(),
      ])
    } else {
      // Suction at the requested strength, fastest pump speed.
      Ok(vec![
        HardwareWriteCmd::new(&[KIIROO_POWERBLOW_PROTOCOL_UUID], Endpoint::Tx, vec![level, 0xff], true).into(),
      ])
    }
  }
}
