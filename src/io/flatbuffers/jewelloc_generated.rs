// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum Unk0Offset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Unk0<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Unk0<'a> {
  type Inner = Unk0<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Unk0<'a> {
  pub const VT_UNK0: flatbuffers::VOffsetT = 4;
  pub const VT_UNK1: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Unk0 { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args Unk0Args
  ) -> flatbuffers::WIPOffset<Unk0<'bldr>> {
    let mut builder = Unk0Builder::new(_fbb);
    builder.add_unk1(args.unk1);
    builder.add_unk0(args.unk0);
    builder.finish()
  }

  pub fn unpack(&self) -> Unk0T {
    let unk0 = self.unk0();
    let unk1 = self.unk1();
    Unk0T {
      unk0,
      unk1,
    }
  }

  #[inline]
  pub fn unk0(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(Unk0::VT_UNK0, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk1(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(Unk0::VT_UNK1, Some(0.0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Unk0<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("unk0", Self::VT_UNK0, false)?
     .visit_field::<f32>("unk1", Self::VT_UNK1, false)?
     .finish();
    Ok(())
  }
}
pub struct Unk0Args {
    pub unk0: u32,
    pub unk1: f32,
}
impl<'a> Default for Unk0Args {
  #[inline]
  fn default() -> Self {
    Unk0Args {
      unk0: 0,
      unk1: 0.0,
    }
  }
}

pub struct Unk0Builder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> Unk0Builder<'a, 'b> {
  #[inline]
  pub fn add_unk0(&mut self, unk0: u32) {
    self.fbb_.push_slot::<u32>(Unk0::VT_UNK0, unk0, 0);
  }
  #[inline]
  pub fn add_unk1(&mut self, unk1: f32) {
    self.fbb_.push_slot::<f32>(Unk0::VT_UNK1, unk1, 0.0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> Unk0Builder<'a, 'b> {
    let start = _fbb.start_table();
    Unk0Builder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Unk0<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Unk0<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Unk0");
      ds.field("unk0", &self.unk0());
      ds.field("unk1", &self.unk1());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Unk0T {
  pub unk0: u32,
  pub unk1: f32,
}
impl Default for Unk0T {
  fn default() -> Self {
    Self {
      unk0: 0,
      unk1: 0.0,
    }
  }
}
impl Unk0T {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Unk0<'b>> {
    let unk0 = self.unk0;
    let unk1 = self.unk1;
    Unk0::create(_fbb, &Unk0Args{
      unk0,
      unk1,
    })
  }
}
pub enum Unk1Offset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Unk1<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Unk1<'a> {
  type Inner = Unk1<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Unk1<'a> {
  pub const VT_RES0: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Unk1 { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args Unk1Args
  ) -> flatbuffers::WIPOffset<Unk1<'bldr>> {
    let mut builder = Unk1Builder::new(_fbb);
    builder.add_res0(args.res0);
    builder.finish()
  }

  pub fn unpack(&self) -> Unk1T {
    let res0 = self.res0();
    Unk1T {
      res0,
    }
  }

  #[inline]
  pub fn res0(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(Unk1::VT_RES0, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Unk1<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("res0", Self::VT_RES0, false)?
     .finish();
    Ok(())
  }
}
pub struct Unk1Args {
    pub res0: u32,
}
impl<'a> Default for Unk1Args {
  #[inline]
  fn default() -> Self {
    Unk1Args {
      res0: 0,
    }
  }
}

pub struct Unk1Builder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> Unk1Builder<'a, 'b> {
  #[inline]
  pub fn add_res0(&mut self, res0: u32) {
    self.fbb_.push_slot::<u32>(Unk1::VT_RES0, res0, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> Unk1Builder<'a, 'b> {
    let start = _fbb.start_table();
    Unk1Builder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Unk1<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Unk1<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Unk1");
      ds.field("res0", &self.res0());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Unk1T {
  pub res0: u32,
}
impl Default for Unk1T {
  fn default() -> Self {
    Self {
      res0: 0,
    }
  }
}
impl Unk1T {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Unk1<'b>> {
    let res0 = self.res0;
    Unk1::create(_fbb, &Unk1Args{
      res0,
    })
  }
}
pub enum PositionOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Position<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Position<'a> {
  type Inner = Position<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Position<'a> {
  pub const VT_X: flatbuffers::VOffsetT = 4;
  pub const VT_Y: flatbuffers::VOffsetT = 6;
  pub const VT_Z: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Position { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PositionArgs
  ) -> flatbuffers::WIPOffset<Position<'bldr>> {
    let mut builder = PositionBuilder::new(_fbb);
    builder.add_z(args.z);
    builder.add_y(args.y);
    builder.add_x(args.x);
    builder.finish()
  }

  pub fn unpack(&self) -> PositionT {
    let x = self.x();
    let y = self.y();
    let z = self.z();
    PositionT {
      x,
      y,
      z,
    }
  }

  #[inline]
  pub fn x(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(Position::VT_X, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn y(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(Position::VT_Y, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn z(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(Position::VT_Z, Some(0.0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Position<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<f32>("x", Self::VT_X, false)?
     .visit_field::<f32>("y", Self::VT_Y, false)?
     .visit_field::<f32>("z", Self::VT_Z, false)?
     .finish();
    Ok(())
  }
}
pub struct PositionArgs {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl<'a> Default for PositionArgs {
  #[inline]
  fn default() -> Self {
    PositionArgs {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
}

pub struct PositionBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PositionBuilder<'a, 'b> {
  #[inline]
  pub fn add_x(&mut self, x: f32) {
    self.fbb_.push_slot::<f32>(Position::VT_X, x, 0.0);
  }
  #[inline]
  pub fn add_y(&mut self, y: f32) {
    self.fbb_.push_slot::<f32>(Position::VT_Y, y, 0.0);
  }
  #[inline]
  pub fn add_z(&mut self, z: f32) {
    self.fbb_.push_slot::<f32>(Position::VT_Z, z, 0.0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PositionBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PositionBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Position<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Position<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Position");
      ds.field("x", &self.x());
      ds.field("y", &self.y());
      ds.field("z", &self.z());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct PositionT {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}
impl Default for PositionT {
  fn default() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
}
impl PositionT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Position<'b>> {
    let x = self.x;
    let y = self.y;
    let z = self.z;
    Position::create(_fbb, &PositionArgs{
      x,
      y,
      z,
    })
  }
}
pub enum JewelLocOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct JewelLoc<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for JewelLoc<'a> {
  type Inner = JewelLoc<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> JewelLoc<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_UNK0: flatbuffers::VOffsetT = 6;
  pub const VT_UNK1: flatbuffers::VOffsetT = 8;
  pub const VT_POSITION: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    JewelLoc { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args JewelLocArgs<'args>
  ) -> flatbuffers::WIPOffset<JewelLoc<'bldr>> {
    let mut builder = JewelLocBuilder::new(_fbb);
    if let Some(x) = args.position { builder.add_position(x); }
    if let Some(x) = args.unk1 { builder.add_unk1(x); }
    if let Some(x) = args.unk0 { builder.add_unk0(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }

  pub fn unpack(&self) -> JewelLocT {
    let name = self.name().map(|x| {
      x.to_string()
    });
    let unk0 = self.unk0().map(|x| {
      Box::new(x.unpack())
    });
    let unk1 = self.unk1().map(|x| {
      Box::new(x.unpack())
    });
    let position = self.position().map(|x| {
      Box::new(x.unpack())
    });
    JewelLocT {
      name,
      unk0,
      unk1,
      position,
    }
  }

  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(JewelLoc::VT_NAME, None)}
  }
  #[inline]
  pub fn unk0(&self) -> Option<Unk0<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<Unk0>>(JewelLoc::VT_UNK0, None)}
  }
  #[inline]
  pub fn unk1(&self) -> Option<Unk1<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<Unk1>>(JewelLoc::VT_UNK1, None)}
  }
  #[inline]
  pub fn position(&self) -> Option<Position<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<Position>>(JewelLoc::VT_POSITION, None)}
  }
}

impl flatbuffers::Verifiable for JewelLoc<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<Unk0>>("unk0", Self::VT_UNK0, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<Unk1>>("unk1", Self::VT_UNK1, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<Position>>("position", Self::VT_POSITION, false)?
     .finish();
    Ok(())
  }
}
pub struct JewelLocArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk0: Option<flatbuffers::WIPOffset<Unk0<'a>>>,
    pub unk1: Option<flatbuffers::WIPOffset<Unk1<'a>>>,
    pub position: Option<flatbuffers::WIPOffset<Position<'a>>>,
}
impl<'a> Default for JewelLocArgs<'a> {
  #[inline]
  fn default() -> Self {
    JewelLocArgs {
      name: None,
      unk0: None,
      unk1: None,
      position: None,
    }
  }
}

pub struct JewelLocBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> JewelLocBuilder<'a, 'b> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(JewelLoc::VT_NAME, name);
  }
  #[inline]
  pub fn add_unk0(&mut self, unk0: flatbuffers::WIPOffset<Unk0<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Unk0>>(JewelLoc::VT_UNK0, unk0);
  }
  #[inline]
  pub fn add_unk1(&mut self, unk1: flatbuffers::WIPOffset<Unk1<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Unk1>>(JewelLoc::VT_UNK1, unk1);
  }
  #[inline]
  pub fn add_position(&mut self, position: flatbuffers::WIPOffset<Position<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Position>>(JewelLoc::VT_POSITION, position);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> JewelLocBuilder<'a, 'b> {
    let start = _fbb.start_table();
    JewelLocBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<JewelLoc<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for JewelLoc<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("JewelLoc");
      ds.field("name", &self.name());
      ds.field("unk0", &self.unk0());
      ds.field("unk1", &self.unk1());
      ds.field("position", &self.position());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct JewelLocT {
  pub name: Option<String>,
  pub unk0: Option<Box<Unk0T>>,
  pub unk1: Option<Box<Unk1T>>,
  pub position: Option<Box<PositionT>>,
}
impl Default for JewelLocT {
  fn default() -> Self {
    Self {
      name: None,
      unk0: None,
      unk1: None,
      position: None,
    }
  }
}
impl JewelLocT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<JewelLoc<'b>> {
    let name = self.name.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let unk0 = self.unk0.as_ref().map(|x|{
      x.pack(_fbb)
    });
    let unk1 = self.unk1.as_ref().map(|x|{
      x.pack(_fbb)
    });
    let position = self.position.as_ref().map(|x|{
      x.pack(_fbb)
    });
    JewelLoc::create(_fbb, &JewelLocArgs{
      name,
      unk0,
      unk1,
      position,
    })
  }
}
pub enum JewelLocsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct JewelLocs<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for JewelLocs<'a> {
  type Inner = JewelLocs<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> JewelLocs<'a> {
  pub const VT_LIST: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    JewelLocs { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args JewelLocsArgs<'args>
  ) -> flatbuffers::WIPOffset<JewelLocs<'bldr>> {
    let mut builder = JewelLocsBuilder::new(_fbb);
    if let Some(x) = args.list { builder.add_list(x); }
    builder.finish()
  }

  pub fn unpack(&self) -> JewelLocsT {
    let list = self.list().map(|x| {
      x.iter().map(|t| t.unpack()).collect()
    });
    JewelLocsT {
      list,
    }
  }

  #[inline]
  pub fn list(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<JewelLoc<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<JewelLoc>>>>(JewelLocs::VT_LIST, None)}
  }
}

impl flatbuffers::Verifiable for JewelLocs<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<JewelLoc>>>>("list", Self::VT_LIST, false)?
     .finish();
    Ok(())
  }
}
pub struct JewelLocsArgs<'a> {
    pub list: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<JewelLoc<'a>>>>>,
}
impl<'a> Default for JewelLocsArgs<'a> {
  #[inline]
  fn default() -> Self {
    JewelLocsArgs {
      list: None,
    }
  }
}

pub struct JewelLocsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> JewelLocsBuilder<'a, 'b> {
  #[inline]
  pub fn add_list(&mut self, list: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<JewelLoc<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(JewelLocs::VT_LIST, list);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> JewelLocsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    JewelLocsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<JewelLocs<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for JewelLocs<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("JewelLocs");
      ds.field("list", &self.list());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct JewelLocsT {
  pub list: Option<Vec<JewelLocT>>,
}
impl Default for JewelLocsT {
  fn default() -> Self {
    Self {
      list: None,
    }
  }
}
impl JewelLocsT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<JewelLocs<'b>> {
    let list = self.list.as_ref().map(|x|{
      let w: Vec<_> = x.iter().map(|t| t.pack(_fbb)).collect();_fbb.create_vector(&w)
    });
    JewelLocs::create(_fbb, &JewelLocsArgs{
      list,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `JewelLocs`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_jewel_locs_unchecked`.
pub fn root_as_jewel_locs(buf: &[u8]) -> Result<JewelLocs, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<JewelLocs>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `JewelLocs` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_jewel_locs_unchecked`.
pub fn size_prefixed_root_as_jewel_locs(buf: &[u8]) -> Result<JewelLocs, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<JewelLocs>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `JewelLocs` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_jewel_locs_unchecked`.
pub fn root_as_jewel_locs_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<JewelLocs<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<JewelLocs<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `JewelLocs` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_jewel_locs_unchecked`.
pub fn size_prefixed_root_as_jewel_locs_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<JewelLocs<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<JewelLocs<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a JewelLocs and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `JewelLocs`.
pub unsafe fn root_as_jewel_locs_unchecked(buf: &[u8]) -> JewelLocs {
  flatbuffers::root_unchecked::<JewelLocs>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed JewelLocs and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `JewelLocs`.
pub unsafe fn size_prefixed_root_as_jewel_locs_unchecked(buf: &[u8]) -> JewelLocs {
  flatbuffers::size_prefixed_root_unchecked::<JewelLocs>(buf)
}
#[inline]
pub fn finish_jewel_locs_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<JewelLocs<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_jewel_locs_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<JewelLocs<'a>>) {
  fbb.finish_size_prefixed(root, None);
}