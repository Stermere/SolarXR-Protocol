// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_BODY_PART: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_BODY_PART: u8 = 21;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_BODY_PART: [BodyPart; 20] = [
  BodyPart::NONE,
  BodyPart::HEAD,
  BodyPart::NECK,
  BodyPart::CHEST,
  BodyPart::WAIST,
  BodyPart::HIP,
  BodyPart::LEFT_UPPER_LEG,
  BodyPart::RIGHT_UPPER_LEG,
  BodyPart::LEFT_LOWER_LEG,
  BodyPart::RIGHT_LOWER_LEG,
  BodyPart::LEFT_FOOT,
  BodyPart::RIGHT_FOOT,
  BodyPart::LEFT_LOWER_ARM,
  BodyPart::RIGHT_LOWER_ARM,
  BodyPart::LEFT_UPPER_ARM,
  BodyPart::RIGHT_UPPER_ARM,
  BodyPart::LEFT_HAND,
  BodyPart::RIGHT_HAND,
  BodyPart::LEFT_SHOULDER,
  BodyPart::RIGHT_SHOULDER,
];

/// Different parts of the body. Roughly maps to each possible bone in the skeleton.
/// These are *NOT* the trackers.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BodyPart(pub u8);
#[allow(non_upper_case_globals)]
impl BodyPart {
  pub const NONE: Self = Self(0);
  pub const HEAD: Self = Self(1);
  pub const NECK: Self = Self(2);
  pub const CHEST: Self = Self(3);
  pub const WAIST: Self = Self(4);
  pub const HIP: Self = Self(5);
  pub const LEFT_UPPER_LEG: Self = Self(6);
  pub const RIGHT_UPPER_LEG: Self = Self(7);
  pub const LEFT_LOWER_LEG: Self = Self(8);
  pub const RIGHT_LOWER_LEG: Self = Self(9);
  pub const LEFT_FOOT: Self = Self(10);
  pub const RIGHT_FOOT: Self = Self(11);
  pub const LEFT_LOWER_ARM: Self = Self(14);
  pub const RIGHT_LOWER_ARM: Self = Self(15);
  pub const LEFT_UPPER_ARM: Self = Self(16);
  pub const RIGHT_UPPER_ARM: Self = Self(17);
  pub const LEFT_HAND: Self = Self(18);
  pub const RIGHT_HAND: Self = Self(19);
  pub const LEFT_SHOULDER: Self = Self(20);
  pub const RIGHT_SHOULDER: Self = Self(21);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 21;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::HEAD,
    Self::NECK,
    Self::CHEST,
    Self::WAIST,
    Self::HIP,
    Self::LEFT_UPPER_LEG,
    Self::RIGHT_UPPER_LEG,
    Self::LEFT_LOWER_LEG,
    Self::RIGHT_LOWER_LEG,
    Self::LEFT_FOOT,
    Self::RIGHT_FOOT,
    Self::LEFT_LOWER_ARM,
    Self::RIGHT_LOWER_ARM,
    Self::LEFT_UPPER_ARM,
    Self::RIGHT_UPPER_ARM,
    Self::LEFT_HAND,
    Self::RIGHT_HAND,
    Self::LEFT_SHOULDER,
    Self::RIGHT_SHOULDER,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::HEAD => Some("HEAD"),
      Self::NECK => Some("NECK"),
      Self::CHEST => Some("CHEST"),
      Self::WAIST => Some("WAIST"),
      Self::HIP => Some("HIP"),
      Self::LEFT_UPPER_LEG => Some("LEFT_UPPER_LEG"),
      Self::RIGHT_UPPER_LEG => Some("RIGHT_UPPER_LEG"),
      Self::LEFT_LOWER_LEG => Some("LEFT_LOWER_LEG"),
      Self::RIGHT_LOWER_LEG => Some("RIGHT_LOWER_LEG"),
      Self::LEFT_FOOT => Some("LEFT_FOOT"),
      Self::RIGHT_FOOT => Some("RIGHT_FOOT"),
      Self::LEFT_LOWER_ARM => Some("LEFT_LOWER_ARM"),
      Self::RIGHT_LOWER_ARM => Some("RIGHT_LOWER_ARM"),
      Self::LEFT_UPPER_ARM => Some("LEFT_UPPER_ARM"),
      Self::RIGHT_UPPER_ARM => Some("RIGHT_UPPER_ARM"),
      Self::LEFT_HAND => Some("LEFT_HAND"),
      Self::RIGHT_HAND => Some("RIGHT_HAND"),
      Self::LEFT_SHOULDER => Some("LEFT_SHOULDER"),
      Self::RIGHT_SHOULDER => Some("RIGHT_SHOULDER"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for BodyPart {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for BodyPart {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for BodyPart {
    type Output = BodyPart;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for BodyPart {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for BodyPart {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for BodyPart {}
