use std::{
    borrow::Cow,
    io::{self, Read, Seek, SeekFrom},
};

use byteorder::LittleEndian;

trait ReadExt: Read {
    fn read_u8(&mut self) -> io::Result<u8> {
        <Self as byteorder::ReadBytesExt>::read_u8(self)
    }

    fn read_i8(&mut self) -> io::Result<i8> {
        <Self as byteorder::ReadBytesExt>::read_i8(self)
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        <Self as byteorder::ReadBytesExt>::read_u16::<LittleEndian>(self)
    }

    fn read_i16(&mut self) -> io::Result<i16> {
        <Self as byteorder::ReadBytesExt>::read_i16::<LittleEndian>(self)
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        <Self as byteorder::ReadBytesExt>::read_u32::<LittleEndian>(self)
    }

    fn read_i32(&mut self) -> io::Result<i32> {
        <Self as byteorder::ReadBytesExt>::read_i32::<LittleEndian>(self)
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        <Self as byteorder::ReadBytesExt>::read_u64::<LittleEndian>(self)
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        <Self as byteorder::ReadBytesExt>::read_i64::<LittleEndian>(self)
    }

    fn read_wcstring(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        loop {
            let x = self.read_u16()?;
            if x == 0 {
                break;
            }
            buffer.push(x);
        }
        Ok(String::from_utf16_lossy(&buffer))
    }
}

impl<R: Read> ReadExt for R {}

pub struct Point {
    pub is_empty: bool,
    pub x: i32,
    pub y: i32,
}

pub struct LevelHeader {
    pub start_y: u8,
    pub goal_y: u8,
    pub goal_x: i16,
    pub timer: u16,
    pub clear_ca: u16,
    pub date_year: u16,
    pub date_mon: u8,
    pub date_day: u8,
    pub date_hour: u8,
    pub date_minute: u8,
    pub autoscroll_speed: u8,
    pub clear_cc: u8,
    pub clear_crc: u32,
    pub game_version: u32,
    pub m_flag: u32,
    pub clear_attempts: u32,
    pub clear_time: u32,
    pub creation_id: u32,
    pub upload_id: u64,
    pub clear_version: u32,
    pub game_style: u16,
    pub name: String,
    pub description: String,
}

impl LevelHeader {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;

        let start_y = reader.read_u8()?;
        let goal_y = reader.read_u8()?;
        let goal_x = reader.read_i16()?;
        let timer = reader.read_u16()?;
        let clear_ca = reader.read_u16()?;
        let date_year = reader.read_u16()?;
        let date_mon = reader.read_u8()?;
        let date_day = reader.read_u8()?;
        let date_hour = reader.read_u8()?;
        let date_minute = reader.read_u8()?;
        let autoscroll_speed = reader.read_u8()?;
        let clear_cc = reader.read_u8()?;
        let clear_crc = reader.read_u32()?;
        let game_version = reader.read_u32()?;
        let m_flag = reader.read_u32()?;
        let clear_attempts = reader.read_u32()?;
        let clear_time = reader.read_u32()?;
        let creation_id = reader.read_u32()?;
        let upload_id = reader.read_u64()?;
        let clear_version = reader.read_u32()?;

        reader.seek(SeekFrom::Start(start + 0xf1))?;
        let game_style = reader.read_u16()?;

        reader.seek(SeekFrom::Start(start + 0xf3 + 1))?;
        let name = reader.read_wcstring()?;
        reader.seek(SeekFrom::Start(start + 0x135 + 1))?;
        let description = reader.read_wcstring()?;

        Ok(Self {
            start_y,
            goal_y,
            goal_x,
            timer,
            clear_ca,
            date_year,
            date_mon,
            date_day,
            date_hour,
            date_minute,
            autoscroll_speed,
            clear_cc,
            clear_crc,
            game_version,
            m_flag,
            clear_attempts,
            clear_time,
            creation_id,
            upload_id,
            clear_version,
            game_style,
            name,
            description,
        })
    }

    pub fn game_style_str(&self) -> Option<&'static str> {
        num_to_game_style(self.game_style)
    }

    pub fn clear_condition_category_str(&self) -> Option<&'static str> {
        num_to_clear_condition_category(self.clear_cc)
    }

    pub fn game_version_str(&self) -> Option<&'static str> {
        num_to_game_version(self.clear_version)
    }

    pub fn clear_condition_str(&self) -> Option<Cow<'static, str>> {
        num_to_clear_condition(self.clear_crc, self.clear_ca)
    }

    pub fn autoscroll_speed_str(&self) -> Option<&'static str> {
        num_to_autoscroll_speed(self.autoscroll_speed)
    }
}

pub struct MapHeader {
    pub theme: u8,
    pub autoscroll_type: u8,
    pub bor_flag: u8,
    pub ori: u8,
    pub liq_e_height: u8,
    pub liq_mode: u8,
    pub liq_speed: u8,
    pub liq_s_height: u8,
    pub b_or_r: u32,
    pub b_or_t: u32,
    pub b_or_l: u32,
    pub b_or_b: u32,
    pub flag: u32,
    pub object_count: u32,
    pub sound_count: u32,
    pub snake_count: u32,
    pub clear_pipe_count: u32,
    pub creeper_count: u32,
    pub iblk_count: u32,
    pub track_block_count: u32,
    pub ground_count: u32,
    pub track_count: u32,
    pub ice_count: u32,
}

impl MapHeader {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let theme = reader.read_u8()?;
        let autoscroll_type = reader.read_u8()?;
        let bor_flag = reader.read_u8()?;
        let ori = reader.read_u8()?;
        let liq_e_height = reader.read_u8()?;
        let liq_mode = reader.read_u8()?;
        let liq_speed = reader.read_u8()?;
        let liq_s_height = reader.read_u8()?;
        let b_or_r = reader.read_u32()?;
        let b_or_t = reader.read_u32()?;
        let b_or_l = reader.read_u32()?;
        let b_or_b = reader.read_u32()?;
        let flag = reader.read_u32()?;
        let object_count = reader.read_u32()?;
        let sound_count = reader.read_u32()?;
        let snake_count = reader.read_u32()?;
        let clear_pipe_count = reader.read_u32()?;
        let creeper_count = reader.read_u32()?;
        let iblk_count = reader.read_u32()?;
        let track_block_count = reader.read_u32()?;
        reader.seek(SeekFrom::Start(start + 0x3c))?;
        let ground_count = reader.read_u32()?;
        let track_count = reader.read_u32()?;
        let ice_count = reader.read_u32()?;

        Ok(Self {
            theme,
            autoscroll_type,
            bor_flag,
            ori,
            liq_e_height,
            liq_mode,
            liq_speed,
            liq_s_height,
            b_or_r,
            b_or_t,
            b_or_l,
            b_or_b,
            flag,
            object_count,
            sound_count,
            snake_count,
            clear_pipe_count,
            creeper_count,
            iblk_count,
            track_block_count,
            ground_count,
            track_count,
            ice_count,
        })
    }

    pub fn theme_str(&self) -> Option<&'static str> {
        num_to_theme(self.theme)
    }

    pub fn autoscroll_type_str(&self) -> Option<&'static str> {
        num_to_autoscroll_type(self.autoscroll_type)
    }

    pub fn boundary_type_str(&self) -> Option<&'static str> {
        num_to_boundary_type(self.bor_flag)
    }

    pub fn orientation_str(&self) -> Option<&'static str> {
        num_to_orientation(self.ori)
    }

    pub fn liquid_mode_str(&self) -> Option<&'static str> {
        num_to_liquid_mode(self.liq_mode)
    }

    pub fn liquid_speed_str(&self) -> Option<&'static str> {
        num_to_liquid_speed(self.liq_speed)
    }
}

#[derive(Debug)]
pub struct MapObject {
    pub x: i32,
    pub y: i32,
    pub w: u8,
    pub h: u8,
    pub flag: u32,
    pub cflag: u32,
    pub ex: u32,
    pub id: i16,
    pub cid: i16,
    pub lid: i16,
    pub sid: i16,
    pub link_type: u8,
}

impl MapObject {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        reader.seek(SeekFrom::Start(start + 0xa))?;
        let w = reader.read_u8()?;
        let h = reader.read_u8()?;
        let flag = reader.read_u32()?;
        let cflag = reader.read_u32()?;
        let ex = reader.read_u32()?;
        let id = reader.read_i16()?;
        let cid = reader.read_i16()?;
        let lid = reader.read_i16()?;
        let sid = reader.read_i16()?;
        let link_type = 0;
        Ok(Self {
            x,
            y,
            w,
            h,
            flag,
            cflag,
            ex,
            id,
            cid,
            lid,
            sid,
            link_type,
        })
    }

    pub fn name(&self, game_style: u16) -> Option<&'static str> {
        num_to_name(self.id, self.flag, game_style)
    }
}

pub struct MapGround {
    pub x: u8,
    pub y: u8,
    pub id: u8,
    pub bid: u8,
}

impl MapGround {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let x = reader.read_u8()?;
        let y = reader.read_u8()?;
        let id = reader.read_u8()?;
        let bid = reader.read_u8()?;
        Ok(Self { x, y, id, bid })
    }
}

pub struct MapTrack {
    pub un: i16,
    pub flag: u8,
    pub x: u8,
    pub y: u8,
    pub type_: u8,
    pub lid: i16,
    pub k0: u16,
    pub k1: u16,
    pub f0: u8,
    pub f1: u8,
    pub f2: u8,
}

impl MapTrack {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let un = reader.read_i16()?;
        let flag = reader.read_u8()?;
        let tx = reader.read_u8()?;
        let x = if tx == 255 { 0 } else { tx + 1 };
        let ty = reader.read_u8()?;
        let y = if ty == 255 { 0 } else { ty + 1 };
        let type_ = reader.read_u8()?;
        let lid = reader.read_i16()?;
        let k0 = reader.read_u16()?;
        let k1 = reader.read_u16()?;

        let f0 = match type_ {
            0..=7 | 11 | 15 => (k0 / 0x80) as u8 % 2,
            8 => (k1 / 0x80) as u8 % 2,
            9 => (k1 / 0x40) as u8 % 2,
            10 => 1 - ((k0 / 0x4000) as u8 % 2),
            12..=13 => (k1 / 0x800) as u8 % 2,
            14 => (k0 / 0x1000) as u8 % 2,
            _ => 0,
        };
        let f1 = match type_ {
            0..=7 => (k1 / 0x80) as u8 % 2,
            8 | 12 => (k0 / 0x80) as u8 % 2,
            9 | 11 => (k1 / 0x2) as u8 % 2,
            10 => (k1 / 0x40) as u8 % 2,
            13 | 15 => (k0 / 0x1000) as u8 % 2,
            14 => (k1 / 0x800) as u8 % 2,
            _ => 0,
        };
        let f2 = match type_ {
            8 => (k0 / 0x8000) as u8 % 2,
            9..=10 | 13..=14 => (k0 / 0x80) as u8 % 2,
            11 => (k1 / 0x40) as u8 % 2,
            12 => (k0 / 0x1000) as u8 % 2,
            15 => (k1 / 0x800) as u8 % 2,
            _ => 0,
        };

        Ok(Self {
            un,
            flag,
            x,
            y,
            type_,
            lid,
            k0,
            k1,
            f0,
            f1,
            f2,
        })
    }
}

pub struct MapClearPipeNode {
    pub type_: u8,
    pub index: u8,
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
    pub dir: u8,
}

impl MapClearPipeNode {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let type_ = reader.read_u8()?;
        let index = reader.read_u8()?;
        let x = reader.read_u8()?;
        let y = reader.read_u8()?;
        let w = reader.read_u8()?;
        let h = reader.read_u8()?;
        reader.seek(SeekFrom::Start(start + 0xb - 0x4))?;
        let dir = reader.read_u8()?;

        Ok(Self {
            type_,
            index,
            x,
            y,
            w,
            h,
            dir,
        })
    }
}

pub struct MapClearPipe {
    pub index: u8,
    pub node_count: u8,
    pub nodes: Vec<MapClearPipeNode>,
}

impl MapClearPipe {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let index = reader.read_u8()?;
        let node_count = reader.read_u8()?;

        let mut nodes = Vec::with_capacity(node_count as usize);
        for i in 0..node_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x4 + 0x8 * i))?;
            nodes.push(MapClearPipeNode::parse(reader)?);
        }

        Ok(Self {
            index,
            node_count,
            nodes,
        })
    }
}

pub struct MapSnakeBlockNode {
    pub index: u8,
    pub dir: u8,
}

impl MapSnakeBlockNode {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let index = reader.read_u8()?;
        reader.seek(SeekFrom::Start(start + 0x6))?;
        let dir = reader.read_u8()?;

        Ok(Self { index, dir })
    }
}

pub struct MapSnakeBlock {
    pub index: u8,
    pub node_count: u8,
    pub nodes: Vec<MapSnakeBlockNode>,
}

impl MapSnakeBlock {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let index = reader.read_u8()?;
        let node_count = reader.read_u8()?;

        let mut nodes = Vec::with_capacity(node_count as usize);
        for i in 0..node_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x8 * i))?;
            nodes.push(MapSnakeBlockNode::parse(reader)?);
        }

        Ok(Self {
            index,
            node_count,
            nodes,
        })
    }
}

pub struct MapMoveBlockNode {
    pub p0: u8,
    pub p1: u8,
    pub p2: u8,
}

impl MapMoveBlockNode {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let p0 = reader.read_u8()?;
        let p1 = reader.read_u8()?;
        let p2 = reader.read_u8()?;
        Ok(Self { p0, p1, p2 })
    }
}

pub struct MapMoveBlock {
    pub index: u8,
    pub node_count: u16,
    pub nodes: Vec<MapMoveBlockNode>,
}

impl MapMoveBlock {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let index = reader.read_u8()?;
        let node_count = reader.read_u16()?;
        let mut nodes = Vec::with_capacity(node_count as usize);
        for i in 0..node_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x4 - 0x1 + 0x4 * i))?;
            nodes.push(MapMoveBlockNode::parse(reader)?);
        }

        Ok(MapMoveBlock {
            index,
            node_count,
            nodes,
        })
    }
}

pub struct MapCreeper {
    pub index: u8,
    pub node_count: u16,
    pub nodes: Vec<u8>,
}

impl MapCreeper {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let index = reader.read_u8()?;
        let node_count = reader.read_u16()?;
        let mut nodes = Vec::with_capacity(node_count as usize);
        for i in 0..node_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x4 + 0x4 * i))?;
            nodes.push(reader.read_u8()?);
        }

        Ok(Self {
            index,
            node_count,
            nodes,
        })
    }
}

pub struct ObjStr {
    pub obj: String,
    pub flag: String,
    pub state: String,
    pub sub_obj: String,
    pub sub_flag: String,
    pub sub_state: String,
}

pub struct Level {
    pub header: LevelHeader,
    pub overworld: Map,
    pub subworld: Map,
}

impl Level {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;
        let header = LevelHeader::parse(reader)?;
        reader.seek(SeekFrom::Start(start + 0x200))?;
        let overworld = Map::parse(reader)?;
        reader.seek(SeekFrom::Start(start + 0x2e0e0))?;
        let subworld = Map::parse(reader)?;

        Ok(Self {
            header,
            overworld,
            subworld,
        })
    }
}

pub struct Map {
    pub map_header: MapHeader,
    pub clear_pipes: Vec<MapClearPipe>,
    pub snake_blocks: Vec<MapSnakeBlock>,
    pub move_blocks: Vec<MapMoveBlock>,
    pub track_blocks: Vec<MapMoveBlock>,
    pub creepers: Vec<MapCreeper>,
    pub objects: Vec<MapObject>,
    pub ground: Vec<MapGround>,
    pub ice: Vec<MapGround>,
    pub tracks: Vec<MapTrack>,
}

impl Map {
    fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let start = reader.stream_position()?;

        let map_header = MapHeader::parse(reader)?;

        let mut objects = Vec::with_capacity(map_header.object_count as usize);
        for i in 0..map_header.object_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x48 + 0x20 * i))?;
            objects.push(MapObject::parse(reader)?);
        }
        //TODO sort objects (not super important)

        let mut snake_blocks = Vec::with_capacity(map_header.snake_count as usize);
        for i in 0..map_header.snake_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x149f8 + 0x3c4 * i))?;
            snake_blocks.push(MapSnakeBlock::parse(reader)?);
        }

        let mut clear_pipes = Vec::with_capacity(map_header.clear_pipe_count as usize);
        for i in 0..map_header.clear_pipe_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x15ccc + 0x124 * i))?;
            clear_pipes.push(MapClearPipe::parse(reader)?);
        }

        let mut creepers = Vec::with_capacity(map_header.creeper_count as usize);
        for i in 0..map_header.creeper_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x240ec + 0x1 + 0x54 * i))?;
            creepers.push(MapCreeper::parse(reader)?);
        }

        let mut move_blocks = Vec::with_capacity(map_header.iblk_count as usize);
        for i in 0..map_header.iblk_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x24434 + 0x1 + 0x2c * i))?;
            move_blocks.push(MapMoveBlock::parse(reader)?);
        }

        let mut track_blocks = Vec::with_capacity(map_header.track_block_count as usize);
        for i in 0..map_header.track_block_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x245ec + 0x1 + 0x2c * i))?;
            track_blocks.push(MapMoveBlock::parse(reader)?);
        }

        let mut ground = Vec::with_capacity(map_header.ground_count as usize);
        for i in 0..map_header.ground_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x247a4 + 0x4 * i))?;
            ground.push(MapGround::parse(reader)?);
        }

        let mut tracks = Vec::with_capacity(map_header.track_count as usize);
        for i in 0..map_header.track_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x28624 + 0x0 + 0xc * i))?;
            tracks.push(MapTrack::parse(reader)?);
        }

        let mut ice = Vec::with_capacity(map_header.ice_count as usize);
        for i in 0..map_header.ice_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x2cc74 + 0x0 + 0x4 * i))?;
            ice.push(MapGround::parse(reader)?);
        }

        Ok(Self {
            map_header,
            clear_pipes,
            snake_blocks,
            move_blocks,
            track_blocks,
            creepers,
            objects,
            ground,
            ice,
            tracks,
        })
    }
}

fn num_to_game_style(x: u16) -> Option<&'static str> {
    match x {
        GAME_STYLE_SMB1 => Some("SMB1"),
        GAME_STYLE_SMB3 => Some("SMB3"),
        GAME_STYLE_SMW => Some("SMW"),
        GAME_STYLE_NSMBU => Some("NSMBU"),
        GAME_STYLE_SM3DW => Some("SM3DW"),
        _ => None,
    }
}

const GAME_STYLE_SMB1: u16 = 12621;
const GAME_STYLE_SMB3: u16 = 13133;
const GAME_STYLE_SMW: u16 = 22349;
const GAME_STYLE_NSMBU: u16 = 21847;
const GAME_STYLE_SM3DW: u16 = 22323;

fn num_to_clear_condition_category(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("None"),
        1 => Some("Parts"),
        2 => Some("Status"),
        3 => Some("Actions"),
        _ => None,
    }
}

fn num_to_game_version(x: u32) -> Option<&'static str> {
    match x {
        0 => Some("1.0.0"),
        1 => Some("1.0.1"),
        2 => Some("1.1.0"),
        3 => Some("2.0.0"),
        4 => Some("3.0.0"),
        5 => Some("3.0.1"),
        33 => Some("What"),
        _ => None,
    }
}

fn num_to_clear_condition(x: u32, y: u16) -> Option<Cow<'static, str>> {
    match x {
        0 => Some("None".into()),
        137525990 => Some("Reach the goal without landing after leaving the ground.".into()),
        199585683 => Some(format!("Reach the goal after defeating at least/all {} Mechakoopa(s).", y).into()),
        272349836 => Some(format!("Reach the goal after defeating at least/all {} Cheep Cheep(s).", y).into()),
        375673178 => Some("Reach the goal without taking damage.".into()),
        426197923 => Some("Reach the goal as Boomerang Mario.".into()),
        436833616 => Some("Reach the goal while wearing a Shoe.".into()),
        713979835 => Some("Reach the goal as Fire Mario.".into()),
        744927294 => Some("Reach the goal as Frog Mario.".into()),
        751004331 => Some(format!("Reach the goal after defeating at least/all {} Larry(s).", y).into()),
        900050759 => Some("Reach the goal as Raccoon Mario.".into()),
        947659466 => Some(format!("Reach the goal after defeating at least/all {} Blooper(s).", y).into()),
        976173462 => Some("Reach the goal as Propeller Mario.".into()),
        994686866 => Some("Reach the goal while wearing a Propeller Box.".into()),
        998904081 => Some(format!("Reach the goal after defeating at least/all {} Spike(s).", y).into()),
        1008094897 => Some(format!("Reach the goal after defeating at least/all {} Boom Boom(s).", y).into()),
        1051433633 => Some("Reach the goal while holding a Koopa Shell.".into()),
        1061233896 => Some(format!("Reach the goal after defeating at least/all {} Porcupuffer(s).", y).into()),
        1062253843 => Some(format!("Reach the goal after defeating at least/all {} Charvaargh(s).", y).into()),
        1079889509 => Some(format!("Reach the goal after defeating at least/all {} Bullet Bill(s).", y).into()),
        1080535886 => Some(format!("Reach the goal after defeating at least/all {} Bully/Bullies.", y).into()),
        1151250770 => Some("Reach the goal while wearing a Goomba Mask.".into()),
        1182464856 => Some(format!("Reach the goal after defeating at least/all {} Hop-Chops.", y).into()),
        1219761531 => Some(
            format!("Reach the goal while holding a Red POW Block. OR Reach the goal after activating at least/all {} Red POW Block(s).", y).into()),
        1221661152 => Some(format!("Reach the goal after defeating at least/all {} Bob-omb(s).", y).into()),
        1259427138 => Some(format!("Reach the goal after defeating at least/all {} Spiny/Spinies.", y).into()),
        1268255615 => Some(format!("Reach the goal after defeating at least/all {} Bowser(s)/Meowser(s).", y).into()),
        1279580818 => Some(format!("Reach the goal after defeating at least/all {} Ant Trooper(s).", y).into()),
        1283945123 => Some("Reach the goal on a Lakitu's Cloud.".into()),
        1344044032 => Some(format!("Reach the goal after defeating at least/all {} Boo(s).", y).into()),
        1425973877 => Some(format!("Reach the goal after defeating at least/all {} Roy(s).", y).into()),
        1429902736 => Some("Reach the goal while holding a Trampoline.".into()),
        1431944825 => Some(format!("Reach the goal after defeating at least/all {} Morton(s).", y).into()),
        1446467058 => Some(format!("Reach the goal after defeating at least/all {} Fish Bone(s).", y).into()),
        1510495760 => Some(format!("Reach the goal after defeating at least/all {} Monty Mole(s).", y).into()),
        1656179347 => Some(format!("Reach the goal after picking up at least/all {} 1-Up Mushroom(s).", y).into()),
        1665820273 => Some(format!("Reach the goal after defeating at least/all {} Hammer Bro(s.).", y).into()),
        1676924210 => Some(
            format!("Reach the goal after hitting at least/all {} P Switch(es). OR Reach the goal while holding a P Switch.", y).into()),
        1715960804 => Some(
            format!("Reach the goal after activating at least/all {} POW Block(s). OR Reach the goal while holding a POW Block.", y).into()),
        1724036958 => Some(format!("Reach the goal after defeating at least/all {} Angry Sun(s).", y).into()),
        1730095541 => Some(format!("Reach the goal after defeating at least/all {} Pokey(s).", y).into()),
        1780278293 => Some("Reach the goal as Superball Mario.".into()),
        1839897151 => Some(format!("Reach the goal after defeating at least/all {} Pom Pom(s).", y).into()),
        1969299694 => Some(format!("Reach the goal after defeating at least/all {} Peepa(s).", y).into()),
        2035052211 => Some(format!("Reach the goal after defeating at least/all {} Lakitu(s).", y).into()),
        2038503215 => Some(format!("Reach the goal after defeating at least/all {} Lemmy(s).", y).into()),
        2048033177 => Some(format!("Reach the goal after defeating at least/all {} Lava Bubble(s).", y).into()),
        2076496776 => Some("Reach the goal while wearing a Bullet Bill Mask.".into()),
        2089161429 => Some("Reach the goal as Big Mario.".into()),
        2111528319 => Some("Reach the goal as Cat Mario.".into()),
        2131209407 => Some(format!("Reach the goal after defeating at least/all {} Goomba(s)/Galoomba(s).", y).into()),
        2139645066 => Some(format!("Reach the goal after defeating at least/all {} Thwomp(s).", y).into()),
        2259346429 => Some(format!("Reach the goal after defeating at least/all {} Iggy(s).", y).into()),
        2549654281 => Some("Reach the goal while wearing a Dry Bones Shell.".into()),
        2694559007 => Some(format!("Reach the goal after defeating at least/all {} Sledge Bro(s.).", y).into()),
        2746139466 => Some(format!("Reach the goal after defeating at least/all {} Rocky Wrench(es).", y).into()),
        2749601092 => Some(format!("Reach the goal after grabbing at least/all {} 50-Coin(s).", y).into()),
        2855236681 => Some("Reach the goal as Flying Squirrel Mario.".into()),
        3036298571 => Some("Reach the goal as Buzzy Mario.".into()),
        3074433106 => Some("Reach the goal as Builder Mario.".into()),
        3146932243 => Some("Reach the goal as Cape Mario.".into()),
        3174413484 => Some(format!("Reach the goal after defeating at least/all {} Wendy(s).", y).into()),
        3206222275 => Some("Reach the goal while wearing a Cannon Box.".into()),
        3314955857 => Some("Reach the goal as Link.".into()),
        3342591980 => Some("Reach the goal while you have Super Star invincibility.".into()),
        3346433512 => Some(format!("Reach the goal after defeating at least/all {} Goombrat(s)/Goombud(s).", y).into()),
        3348058176 => Some(format!("Reach the goal after grabbing at least/all {} 10-Coin(s).", y).into()),
        3353006607 => Some(format!("Reach the goal after defeating at least/all {} Buzzy Beetle(s).", y).into()),
        3392229961 => Some(format!("Reach the goal after defeating at least/all {} Bowser Jr.(s).", y).into()),
        3437308486 => Some(format!("Reach the goal after defeating at least/all {} Koopa Troopa(s).", y).into()),
        3459144213 => Some(format!("Reach the goal after defeating at least/all {} Chain Chomp(s).", y).into()),
        3466227835 => Some(format!("Reach the goal after defeating at least/all {} Muncher(s).", y).into()),
        3481362698 => Some(format!("Reach the goal after defeating at least/all {} Wiggler(s).", y).into()),
        3513732174 => Some("Reach the goal as SMB2 Mario.".into()),
        3649647177 => Some("Reach the goal in a Koopa Clown Car/Junior Clown Car.".into()),
        3725246406 => Some("Reach the goal as Spiny Mario.".into()),
        3730243509 => Some("Reach the goal in a Koopa Troopa Car.".into()),
        3748075486 => Some(format!("Reach the goal after defeating at least/all {} Piranha Plant(s)/Jumping Piranha Plant(s).", y).into()),
        3797704544 => Some(format!("Reach the goal after defeating at least/all {} Dry Bones.", y).into()),
        3824561269 => Some(format!("Reach the goal after defeating at least/all {} Stingby/Stingbies.", y).into()),
        3833342952 => Some(format!("Reach the goal after defeating at least/all {} Piranha Creeper(s).", y).into()),
        3842179831 => Some(format!("Reach the goal after defeating at least/all {} Fire Piranha Plant(s).", y).into()),
        3874680510 => Some(format!("Reach the goal after breaking at least/all {} Crates(s).", y).into()),
        3974581191 => Some(format!("Reach the goal after defeating at least/all {} Ludwig(s).", y).into()),
        3977257962 => Some("Reach the goal as Super Mario.".into()),
        4042480826 => Some(format!("Reach the goal after defeating at least/all {} Skipsqueak(s).", y).into()),
        4116396131 => Some(format!("Reach the goal after grabbing at least/all {} Coin(s).", y).into()),
        4117878280 => Some(format!("Reach the goal after defeating at least/all {} Magikoopa(s).", y).into()),
        4122555074 => Some(format!("Reach the goal after grabbing at least/all {} 30-Coin(s).", y).into()),
        4153835197 => Some("Reach the goal as Balloon Mario.".into()),
        4172105156 => Some("Reach the goal while wearing a Red POW Box.".into()),
        4209535561 => Some("Reach the Goal while riding Yoshi.".into()),
        4269094462 => Some(format!("Reach the goal after defeating at least/all {} Spike Top(s).", y).into()),
        4293354249 => Some(format!("Reach the goal after defeating at least/all {} Banzai Bill(s).", y).into()),
        _ => None,
    }
}

fn num_to_theme(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("Overworld"),
        1 => Some("Underground"),
        2 => Some("Castle"),
        3 => Some("Airship"),
        4 => Some("Underwater"),
        5 => Some("Ghost house"),
        6 => Some("Snow"),
        7 => Some("Desert"),
        8 => Some("Sky"),
        9 => Some("Forest"),
        _ => None,
    }
}

fn num_to_autoscroll_speed(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("x1"),
        1 => Some("x2"),
        2 => Some("x3"),
        _ => None,
    }
}

fn num_to_autoscroll_type(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("None"),
        1 => Some("Slow"),
        2 => Some("Normal"),
        3 => Some("Fast"),
        4 => Some("Custom"),
        _ => None,
    }
}

fn num_to_boundary_type(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("Built Above Line"),
        1 => Some("Built Below Line"),
        _ => None,
    }
}

fn num_to_orientation(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("Horizontal"),
        1 => Some("Vertical"),
        _ => None,
    }
}

fn num_to_liquid_mode(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("Static"),
        1 => Some("Rising or Falling"),
        2 => Some("Rising and Falling"),
        _ => None,
    }
}

fn num_to_liquid_speed(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("None"),
        1 => Some("x1"),
        2 => Some("x2"),
        3 => Some("x3"),
        _ => None,
    }
}

fn num_to_name(id: i16, flag: u32, game_style: u16) -> Option<&'static str> {
    let alt_item = (flag & FLAG_ALT_ITEM) != 0;
    let is_p_door = (flag & FLAG_IS_P_DOOR) != 0;
    let is_key_door = (flag & FLAG_IS_KEY_DOOR) != 0;
    match id {
        0 if !alt_item => match game_style {
            GAME_STYLE_SMB1 | GAME_STYLE_SMB3 | GAME_STYLE_NSMBU | GAME_STYLE_SM3DW => {
                Some("Goomba")
            }
            GAME_STYLE_SMW => Some("Galoomba"),
            _ => None,
        },
        0 if alt_item => match game_style {
            GAME_STYLE_SMB1 | GAME_STYLE_SMB3 | GAME_STYLE_NSMBU | GAME_STYLE_SM3DW => {
                Some("Goombrat")
            }
            GAME_STYLE_SMW => Some("Goombud"),
            _ => None,
        },
        1 => Some("Koopa"),
        2 => Some("Piranha Plant"),
        3 => Some("Hammer Bro"),
        4 => Some("Block"),
        5 => Some("? Block"),
        6 => Some("Stone"),
        7 => Some("Hard Block"),
        8 => Some("Coin"),
        9 => Some("Pipe"),
        10 => Some("Trampoline"),
        11 => Some("Lift"),
        12 => Some("Thwomp"),
        13 => Some("Bill Blaster"),
        14 => Some("Mushroom Platform"),
        15 => Some("Bob-omb"),
        16 => Some("Semisolid Platform"),
        17 => Some("Bridge"),
        18 => Some("P Switch"),
        19 => Some("POW"),
        20 if !alt_item => Some("Super Mushroom"),
        20 if alt_item => Some("Master Sword"),
        21 => Some("Donut Block"),
        22 => Some("Cloud"),
        23 => Some("Note Block"),
        24 => Some("Fire Bar"),
        25 => Some("Spiny"),
        26 => Some("Hard Block (Goal)"),
        27 => Some("Goal"),
        28 => Some("Buzzy Beetle"),
        29 => Some("Hidden Block"),
        30 => Some("Lakitu"),
        31 => Some("Lakitu Cloud"),
        32 => Some("Banzai Bill"),
        33 => Some("1-Up Mushroom"),
        34 if !alt_item => Some("Fire Flower"),
        34 if alt_item => Some("Superball Flower"),
        35 => Some("Super Star"),
        36 => Some("Lava Lift"),
        37 => Some("Starting Brick"),
        38 => Some("Starting Arrow"),
        39 => Some("Magikoopa"),
        40 => Some("Spike Top"),
        41 => Some("Boo"),
        42 => Some("Koopa Clown Car"),
        43 => Some("Spike Trap"),
        44 => match game_style {
            GAME_STYLE_SMB1 => Some("Big Mushroom"),
            GAME_STYLE_SMB3 => Some("Super Leaf"),
            GAME_STYLE_SMW => Some("Cape Feather"),
            GAME_STYLE_NSMBU => Some("Propeller Mushroom"),
            GAME_STYLE_SM3DW => Some("Super Bell"),
            _ => None,
        },
        45 => match game_style {
            GAME_STYLE_SMB1 | GAME_STYLE_SMB3 => Some("Shoe Goomba"),
            GAME_STYLE_SMW | GAME_STYLE_NSMBU => Some("Yoshi"),
            _ => None,
        },
        46 => Some("Dry Bones"),
        47 => Some("Cannon"),
        48 => Some("Blooper"),
        49 => Some("Castle Bridge"),
        50 => Some("Hop-Chops"),
        51 => Some("Skipsqueak"),
        52 => Some("Wiggler"),
        53 => Some("Conveyor Belt"),
        54 => Some("Burner"),
        55 if is_p_door => Some("P Warp Door"),
        55 if is_key_door => Some("Key Door"),
        55 => Some("Warp Door"),
        56 => Some("Cheep Cheep"),
        57 => Some("Muncher"),
        58 => Some("Rocky Wrench"),
        59 => Some("Track"),
        60 => Some("Lava Bubble"),
        61 => Some("Chain Chomp"),
        62 => Some("Bowser"),
        63 => Some("Ice Block"),
        64 => Some("Vine"),
        65 => Some("Stingby"),
        66 => Some("Arrow"),
        67 => Some("One-Way Wall"),
        68 => Some("Grinder"),
        69 => Some("Player"),
        70 => Some("Big Coin"),
        71 => Some("Half Collision Platform"),
        72 => Some("Koopa Troopa Car"),
        //73 => Some("Cinobio (not yet named)"),
        74 if !alt_item => Some("Spike"),
        74 if alt_item => Some("Spike Ball"),
        75 => Some("Stone"),
        76 => Some("Twister"),
        77 => Some("Boom Boom"),
        78 => Some("Pokey"),
        79 => Some("P Block"),
        80 => Some("Dash Block"),
        81 => match game_style {
            GAME_STYLE_SMB1 => Some("SMB2 Mushroom"),
            GAME_STYLE_SMB3 => Some("Frog Suit"),
            GAME_STYLE_SMW => Some("Power Balloon"),
            GAME_STYLE_NSMBU => Some("Super Acorn"),
            GAME_STYLE_SM3DW => Some("Boomerang Flower"),
            _ => None,
        },
        82 => Some("Bumper"),
        83 => Some("Skewer"),
        84 => Some("Snake Block"),
        85 => Some("Track Block"),
        86 => Some("Charvaargh"),
        87 => Some("Slight Slope"),
        88 => Some("Steep Slope"),
        89 => Some("Reel Camera"),
        90 => Some("Checkpoint Flag"),
        91 => Some("Seesaw"),
        92 => Some("Pink Coin"),
        93 => Some("Clear Pipe"),
        94 => Some("Conveyor Belt"),
        95 => Some("Key"),
        96 => Some("Ant Trooper"),
        97 => Some("Warp Box"),
        98 => Some("Bowser Jr."),
        99 => Some("ON/OFF Switch"),
        100 => Some("Dotted-Line Block"),
        101 => Some("Water Marker"),
        102 => Some("Monty Mole"),
        103 => Some("Fish Bone"),
        104 => Some("Angry Sun/Moon"),
        105 => Some("Swinging Claw"),
        106 => Some("Tree"),
        107 => Some("Piranha Creeper"),
        108 => Some("Blinking Block"),
        109 => Some("Sound Effect"),
        110 => Some("Spike Block"),
        111 => Some("Mechakoopa"),
        112 => Some("Crate"),
        113 => Some("Mushroom Trampoline"),
        114 => Some("Porcupuffer"),
        //115 => Some("Cinobic"),
        116 => Some("Super Hammer"),
        117 => Some("Bully"),
        118 => Some("Icicle"),
        119 => Some("! Block"),
        120 => Some("Lemmy"),
        121 => Some("Morton"),
        122 => Some("Larry"),
        123 => Some("Wendy"),
        124 => Some("Iggy"),
        125 => Some("Roy"),
        126 => Some("Ludwig"),
        127 => Some("Cannon Box"),
        128 => Some("Propeller Box"),
        129 => Some("Goomba Mask"),
        130 => Some("Bullet Bill Mask"),
        131 => Some("Red POW Box"),
        132 => Some("ON/OFF Trampoline"),
        _ => None,
    }
}

const FLAG_IS_P_DOOR: u32 = 0x0004_0000;
const FLAG_IS_KEY_DOOR: u32 = 0x0008_0000;
const FLAG_ALT_ITEM: u32 = 0x0000_0004;
