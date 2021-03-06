use super::*;

pub struct ProbeResponseFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> ProbeResponseFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }
}
impl<'a> FrameTrait<'a> for ProbeResponseFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> FragmentSequenceTrait<'a> for ProbeResponseFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for ProbeResponseFrame<'a> {}
impl<'a> BeaconFixedParametersTrait<'a> for ProbeResponseFrame<'a> {}
impl<'a> TaggedParametersTrait<'a> for ProbeResponseFrame<'a> {}
