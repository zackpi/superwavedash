use std::rc::Rc;

/// Header
/// The DAT Header stores all the initial information needed to start reading the file
#[derive(Debug, Default)]
pub struct Header {
    /// The DAT file's size in bytes
    pub file_size: u32,
    /// The Body's size in bytes (equivalently, the PointerTable offset)
    pub body_size: u32,
    /// The number of entries in the RelocationTable
    pub reltab_count: u32,
    /// The number of Root nodes
    pub root_count: u32,
    /// The number of Xref nodes
    pub xref_count: u32,
    /// "001B" when used
    pub unknown_0x14: u32,
    /// Padding?
    pub unknown_0x18: u32,
    /// Padding?
    pub unknown_0x1c: u32,
}

/// Root node
/// The node's name can be used to determine its type.
/// See: https://docs.google.com/document/d/1TIFquuPCrFEANPAH_CM7A138SX07BQ3c4Q2Wms5I1OU/edit#heading=h.wxs9ydr2fex1
#[derive(Debug, Default)]
pub struct Root {
    /// Offset to the child node in the Body block.
    pub child: u32,
    /// The offset in the StringTable where this node's name is stored.
    pub name: u32,
}

/// DAT file External Node Reference (Xref node)
/// This offset will be replaced at load time by a pointer to the correct data.
/// The data is usually found in another file whihc must be loaded alongside the current file.
/// There must exist a Root node in an external file with an identical name,
/// so its runtime pointer can be stored in the offset pointed to by "offset"
#[derive(Debug, Default)]
pub struct Xref {
    /// The offset to the referenced node in the Body.
    pub offset: u32,
    /// See: Root.name
    pub name: u32,
}

// Body
// The Body is the main part of the file which holds the data in different data nodes.
// This could be images, meshes, collision data, etc.
// Most of these types of data, or 'nodes' have different formats.

/// Joint
/// A Joint node holds position, scale, and rotation data,
/// and pointers to children which use this.
/// Often these are used for a mesh's skeleton, or to layout a scene or menu.
#[derive(Debug, Default)]
pub struct Joint {
    pub unknown_0x00: u32,
    pub flags: u32,
    pub child: Rc<Joint>,
    pub next: Rc<Joint>,
    pub data: Rc<JointData>,
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub position: [f32; 3],
    pub transform: Rc<Transform>,
    pub unknown_0x3c: u32,
}

#[derive(Debug, Default)]
pub struct JointData {
    pub unknown_0x00: u32,
    pub next: Rc<JointData>,
    pub material: Rc<Material>,
    pub mesh: Rc<Mesh>,
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub class: u32,
    pub next: Rc<Mesh>,
    pub verts: Rc<VertAttr>,
    pub flags: u16,
    pub ndisplay: u16,
    pub display: Rc<Display>,
    pub weight: u32,
}

#[derive(Debug, Default)]
pub struct Material {
    pub class: u32,
    pub render_mode: u32,
    pub texture: Rc<Texture>,
    pub color: Rc<Color>,
    pub render_desc: u32,
    pub pedesc: Rc<PEDesc>,
}

#[derive(Debug, Default)]
pub struct Color {
    pub diffuse: u32,
    pub ambient: u32,
    pub specular: u32,
    pub alpha: f32,
    pub shininess: f32,
}

#[derive(Debug, Default)]
pub struct Texture {
    pub unknown_0x00: u32,
    pub next: Rc<Texture>,
    pub unknown_0x08: u32,
    pub unknown_0x0c: u32,
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub position: [f32; 3],
    pub s_wrap: u32,
    pub t_wrap: u32,
    pub s_scale: u8,
    pub t_scale: u8,
    pub anim_id: u16,
    pub mapping_flags: u32,
    pub blending: u32,
    pub unknown_0x48: u32,
    pub image: Rc<Image>,
    pub palette: Rc<Palette>,
    pub unknown_0x54: u32,
    pub unknown_0x58: Rc<TexUnk>,
}

#[derive(Debug, Default)]
pub struct Image {
    pub data: u32,
    pub width: u16,
    pub height: u16,
    pub format: u32,
}

#[derive(Debug, Default)]
pub struct Palette {
    pub data: u32,
    pub format: u32,
    pub name: u32,
    pub colornum: u16,
    pub padding: u16,
}

#[derive(Debug, Default)]
pub struct Transform {
    pub m: [[u32; 4]; 4],
}

#[derive(Debug, Default)]
pub struct Weight {
    pub joint: Rc<Joint>,
    pub factor: f32,
}

#[derive(Debug, Default)]
pub struct VertAttr {
    pub ident: u32,
    pub usage: u32,
    pub format: u32,
    pub datatype: u32,
    pub scale: u8,
    pub unknown_0x11: u8,
    pub stride: u16,
    pub data: u32,
}

#[derive(Debug, Default)]
pub struct Display {
    pub unknown_0x00: u8,
    pub primitive: u8,
    pub indices: u16,
}

#[derive(Debug, Default)]
pub struct Collision {
    pub spots: u32,
    pub nspots: u32,
    pub links: Rc<Link>,
    pub nlinks: u32,
    pub index_table: [Index; 5],
    pub colldata: Rc<Area>,
    pub ncolldata: u32,
}

#[derive(Debug, Default)]
struct Index {
    pub start: u16,
    pub num: u16,
}

#[derive(Debug, Default)]
pub struct Link {
    pub from_spot: u16,
    pub to_spot: u16,
    pub from: u16,
    pub to: u16,
    pub from_virt: u16,
    pub to_virt: u16,
    pub flags: u32,
}

#[derive(Debug, Default)]
pub struct Area {
    pub indextable: [Index; 4],
    pub unknown_0x10: u32,
    pub bounds: [f32; 4],
    pub low: u16,
    pub nlink: u16,
}

#[derive(Debug, Default)]
pub struct HitBox {
    pub primary: u32,
    pub scale: u16,
    pub z_off: u16,
    pub y_off: u16,
    pub x_off: u16,
    pub physics: u32,
    pub effects: u32,
}

// Body (WIP)
// The following are nodes which have purpose yet to be discovered,
// and are still in the process of being documented.
// They will contain only the structures and any notes.
// It is difficult to tell which offset is the child
// and which is the sibling when two are present
// as both lead to more tree structures,
// so most of the time I call the first one ‘next’
// and the second one (if present) ‘child’.

#[derive(Debug, Default)]
pub struct JointDataUnk1 {
    pub flags: u32,
    pub data1: Rc<JointDataUnk2>,
    pub data2: Rc<JointDataUnk2>,
    pub data3: Rc<JointDataUnk2>,
}

#[derive(Debug, Default)]
pub struct JointDataUnk2 {}

#[derive(Debug, Default)]
pub struct TexUnk {
    pub unknown_0x00: u32,
    pub unknown_0x04: u32,
    pub unknown_0x08: u32,
    pub unknown_0x0c: u32,
    pub unknown_0x10: u32,
    pub unknown_0x14: u32,
    pub unknown_0x18: u32,
    pub unknown_0x1c: u32,
}

#[derive(Debug, Default)]
pub struct MatAnimJoint {
    pub next: Rc<MatAnimJoint>,
    pub child: Rc<MatAnimJoint>,
    pub matanim: MatAnim,
}

#[derive(Debug, Default)]
pub struct MatAnim {
    pub next: Rc<MatAnim>,
    pub aobjdesc: Rc<AObjDesc>,
    pub texanim: Rc<TexAnim>,
    pub renderanim: Rc<RenderAnim>,
}

#[derive(Debug, Default)]
pub struct TexAnim {
    pub next: Rc<TexAnim>,
    pub texmapid: u32,
    pub aobjdesc: Rc<AObjDesc>,
    pub images: u32,
    pub palettes: u32,
    pub nimage: u32,
    pub npalette: u32,
}

#[derive(Debug, Default)]
pub struct AObjDesc {
    pub flags: u32,
    pub end_frame: f32,
    pub fobjdesc: Rc<FObjDesc>,
    pub obj_type: u32,
}

#[derive(Debug, Default)]
pub struct FObjDesc {
    pub next: Rc<FObjDesc>,
    pub str_len: u32,
    pub unknown_0x08: u32,
    pub flags: u32,
    pub string: String,
}

#[derive(Debug, Default)]
pub struct MapHeadA {
    pub mapjoint: Rc<MapJoint>,
    pub nmapjoint: u32,
    pub mapheadb: Rc<MapHeadB>,
    pub nmapheadb: u32,
    pub jointdata: Rc<JointDataUnk1>,
    pub njointdata: u32,
    pub mapheade: Rc<MapHeadE>,
    pub nmapheade: u32,
    pub materials: u32,
    pub nmaterials: u32,
}

#[derive(Debug, Default)]
pub struct MapHeadB {
    pub joint: Rc<Joint>,
    pub mapheadq: Rc<MapHeadQ>,
    pub matanima: Rc<MatAnimA>,
    pub mapheadf: Rc<MapHeadF>,
    pub mapheadc: Rc<MapHeadC>,
    pub mapheadh: Rc<MapHeadH>,
    pub mapplitb: Rc<MapPlitB>,
    pub mapheadd: Rc<MapHeadD>,
    pub half: u32,
    pub nhalf: u32,
    pub allornot: u32,
}

#[derive(Debug, Default)]
pub struct MapHeadC {
    pub unknown_0x00: u32,
    pub unknown_0x04: u32,
    pub unknown_0x08: u32,
    pub unknown_0x0c: u32,
    pub unknown_0x10: u32,
    pub unknown_0x14: u32,
    pub mpca1: f32,
    pub mpca2: f32,
    pub unknown_0x20: u32,
    pub unknown_0x24: u32,
    pub unknown_0x28: u32,
    pub unknown_0x2c: u32,
    pub unknown_0x30: u32,
    pub unknown_0x34: u32,
}

#[derive(Debug, Default)]
pub struct MapHeadD {
    pub unknown_0x00: u32,
    pub unknown_0x04: u32,
    pub unknown_0x08: u16,
    pub unknown_0x0a: u16,
    pub flags: u32,
    pub this: Rc<MapHeadD>,
}

#[derive(Debug, Default)]
pub struct MapHeadE {
    pub mapplitc: Rc<MapPlitC>,
}

#[derive(Debug, Default)]
pub struct MapHeadF {
    pub next: Rc<MapHeadF>,
    pub child: Rc<MapHeadF>,
    pub mapheadg: Rc<MapHeadG>,
}

#[derive(Debug, Default)]
pub struct MapHeadG {
    pub unknown_0x00: u32,
    pub unknown_0x04: u32,
}

#[derive(Debug, Default)]
pub struct MapHeadH {
    pub unknown_0x00: u32,
    pub triplet0: u32,
    pub triplet1: u32,
    pub triplet2: u32,
    pub unknown_0x10: u32,
    pub mapheadc: Rc<MapHeadC>,
    pub this: Rc<MapHeadH>,
}

#[derive(Debug, Default)]
pub struct MapHeadQ {
    pub next: Rc<MapHeadQ>,
    pub child: Rc<MapHeadQ>,
    pub matanimd: Rc<MatAnimD>,
    pub unknown_0x0c: u32,
    pub unknown_0x10: u32,
}

#[derive(Debug, Default)]
pub struct MapJoint {
    pub joint: Rc<Joint>,
    pub half: u32,
    pub nhalf: u32,
}

#[derive(Debug, Default)]
pub struct MapPlitB {
    pub mapplitc: Rc<MapPlitC>,
    pub mapplitd: Rc<MapPlitD>,
}

#[derive(Debug, Default)]
pub struct MapPlitC {
    pub unknown_0x00: u32,
    pub unknown_0x04: u32,
    pub unknown_0x08: u32,
    pub unknown_0x0c: u16,
    pub unknown_0x0e: u16,
    pub flags: u32,
    pub mpca: u32,
    pub mpcb: u32,
}

#[derive(Debug, Default)]
pub struct MapPlitD {
    pub mapplite: Rc<MapPlitE>,
    pub unknown_0x04: u32,
}

#[derive(Debug, Default)]
pub struct MapPlitE {
    pub unknown_0x00: u32,
    pub unknown_0x04: u32,
    pub unknown_0x08: u32,
    pub unknown_0x0c: u32,
}

#[derive(Debug, Default)]
pub struct QuakeMS {
    pub qmsa: u32,
    pub qms: u32,
    pub qmsb: Rc<QuakeMSB>,
    pub unknown_0x0c: u32,
    pub this: Rc<QuakeMS>,
}

#[derive(Debug, Default)]
pub struct QuakeMSB {
    pub unknown_0x00: u32,
    pub unknown_0x04: u32,
    pub qmsc: Rc<QuakeMSC>,
}

#[derive(Debug, Default)]
pub struct QuakeMSC {
    pub next: Rc<QuakeMSC>,
    pub unknown_0x04: u32,
    pub unknown_0x08: u32,
    pub unknown_0x0c: u32,
    pub data: u32,
}

#[derive(Debug, Default)]
pub struct ItemData {
    pub itemtype: u32,
    pub data: u32,
}

#[derive(Debug, Default)]
pub struct GroundParam {
    pub scale: f32,
    pub shadow: u32,
    pub unknown_0x08: u32,
    pub unknown_0x0c: u32,
    pub unknown_0x10: u32,
    pub tilt: u32,
    pub hrot: f32,
    pub vrot: f32,
    pub cam_fix: f32,
    pub unknown_0x24: f32,
    pub cam_smooth: f32,
    pub unknown_0x2c: u32,
    pub pause_min: u32,
    pub pause_init: u32,
    pub pause_max: u32,
    pub pause_up: f32,
    pub pause_left: f32,
    pub pause_right: f32,
    pub pause_down: f32,
    pub unknown_0x4c: u32,
    pub unknown_0x50: u32,
    pub unknown_0x54: f32,
    pub unknown_0x58: f32,
    pub unknown_0x5c: f32,
    pub unknown_0x60: f32,
    pub sub: [Sub; 20],
    pub music: Rc<Music>,
    pub nmusic: u32,
}

#[derive(Debug, Default)]
pub struct Sub {
    pub sub_0x00: u16,
    pub sub_0x02: u16,
}

#[derive(Debug, Default)]
pub struct Music {
    pub stage: u32,
    pub bgm: u32,
    pub bgm_alt: u32,
    pub bgm2: u32,
    pub bgm_alt2: u32,
    pub alt: u16,
    pub alt_change: u16,
    pub unknown_0x18: u32,
    pub unknown_0x20: u32,
    pub unknown_0x24: u32,
    pub unknown_0x28: u32,
    pub unknown_0x2c: u32,
    pub unknown_0x30: u32,
    pub unknown_0x34: u32,
    pub unknown_0x38: u32,
    pub unknown_0x3c: u32,
    pub unknown_0x40: u32,
    pub unknown_0x44: u32,
    pub unknown_0x48: u32,
    pub unknown_0x4c: u32,
    pub unknown_0x50: u32,
    pub unknown_0x54: u32,
    pub unknown_0x58: u32,
    pub unknown_0x5c: u32,
    pub unknown_0x60: u32,
}
