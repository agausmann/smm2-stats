use std::io::{self, Read, Seek, SeekFrom};

use byteorder::LittleEndian;

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
}

pub struct MapObject {
    pub x: i32,
    pub y: i32,
    pub w: u8,
    pub h: u8,
    pub flag: i32,
    pub cflag: i32,
    pub ex: i32,
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
        let flag = reader.read_i32()?;
        let cflag = reader.read_i32()?;
        let ex = reader.read_i32()?;
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
        let overworld = Map::parse(
            reader,
            MapType::Overworld {
                start_y: header.start_y,
                goal_x: header.goal_x,
                goal_y: header.goal_y,
            },
        )?;
        reader.seek(SeekFrom::Start(start + 0x2e0e0))?;
        let subworld = Map::parse(reader, MapType::Subworld)?;

        Ok(Self {
            header,
            overworld,
            subworld,
        })
    }
}

enum MapType {
    Overworld {
        start_y: u8,
        goal_x: i16,
        goal_y: u8,
    },
    Subworld,
}

pub struct Map {
    pub map_header: MapHeader,
    pub clear_pipes: Vec<MapClearPipe>,
    pub snake_blocks: Vec<MapSnakeBlock>,
    pub track_y_points: Vec<Point>,
    pub track_nodes: Vec<Vec<u32>>,
    pub ground_nodes: Vec<Vec<u32>>,
    pub move_blocks: Vec<MapMoveBlock>,
    pub track_blocks: Vec<MapMoveBlock>,
    pub creepers: Vec<MapCreeper>,
    pub object_link_type: Vec<u8>,
    pub objects: Vec<MapObject>,
    pub ground: Vec<MapGround>,
    pub ice: Vec<MapGround>,
    pub tracks: Vec<MapTrack>,
    pub tile_loc: Vec<Vec<Point>>,
    pub pipe_loc: Vec<Vec<Point>>,
    pub ground_loc: Vec<Point>,
    pub obj_loc_data: Vec<Vec<Vec<ObjStr>>>,
}

impl Map {
    fn parse<R: Read + Seek>(reader: &mut R, map_type: MapType) -> io::Result<Self> {
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
        let mut ground_nodes = vec![vec![0; 300]; 300];
        for i in 0..map_header.ground_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x247a4 + 0x4 * i))?;
            let a_ground = MapGround::parse(reader)?;
            ground_nodes[a_ground.x as usize + 1][a_ground.y as usize + 1] = 1;
            ground.push(a_ground);
        }

        if let MapType::Overworld {
            start_y,
            goal_x,
            goal_y,
        } = map_type
        {
            let min = ((goal_x as f32 - 5.0) / 10.0).round() as usize;
            let max = ((goal_x as f32 - 5.0) / 10.0 + 9.0) as usize;
            for j in min..=max {
                for i in 0..goal_y as usize {
                    ground_nodes[j + 1][i + 1] = 1;
                }
            }
            for j in 0..=6 {
                for i in 0..start_y as usize {
                    ground_nodes[j + 1][i + 1] = 1;
                }
            }
        }

        let mut tracks = Vec::with_capacity(map_header.track_count as usize);
        let mut track_nodes =
            vec![vec![0; map_header.b_or_t as usize + 3]; map_header.b_or_r as usize + 3];

        for i in 0..map_header.track_count as u64 {
            reader.seek(SeekFrom::Start(start + 0x28624 + 0x0 + 0xc * i))?;
            let track = MapTrack::parse(reader)?;

            let x = track.x as usize;
            let y = track.y as usize;
            match track.type_ {
                0 => {
                    track_nodes[x][y + 1] += 1;
                    track_nodes[x + 2][y + 1] += 1;
                }
                1 => {
                    track_nodes[x + 1][y + 2] += 1;
                    track_nodes[x + 1][y] += 1;
                }
                2 | 4 | 5 => {
                    track_nodes[x][y + 2] += 1;
                    track_nodes[x + 2][y] += 1;
                }
                3 | 6 | 7 => {
                    track_nodes[x + 2][y + 2] += 1;
                    track_nodes[x][y] += 1;
                }
                8 => {
                    track_nodes[x][y + 2] += 1;
                    track_nodes[x + 4][y] += 1;
                    track_nodes[x + 4][y + 4] += 1;
                }
                9 => {
                    track_nodes[x][y] += 1;
                    track_nodes[x][y + 4] += 1;
                    track_nodes[x + 4][y + 2] += 1;
                }
                10 => {
                    track_nodes[x][y] += 1;
                    track_nodes[x + 2][y + 4] += 1;
                    track_nodes[x + 4][y] += 1;
                }
                11 => {
                    track_nodes[x + 2][y] += 1;
                    track_nodes[x][y + 4] += 1;
                    track_nodes[x + 4][y + 4] += 1;
                }
                12 => {
                    track_nodes[x][y + 2] += 1;
                    track_nodes[x + 4][y] += 1;
                    track_nodes[x + 4][y + 4] += 1;
                }
                13 => {
                    track_nodes[x][y] += 1;
                    track_nodes[x][y + 4] += 1;
                    track_nodes[x + 4][y + 2] += 1;
                }
                14 => {
                    track_nodes[x][y] += 1;
                    track_nodes[x + 4][y] += 1;
                    track_nodes[x + 2][y + 4] += 1;
                }
                15 => {
                    track_nodes[x + 2][y] += 1;
                    track_nodes[x][y + 4] += 1;
                    track_nodes[x + 4][y + 4] += 1;
                }
                _ => {}
            }

            tracks.push(track);
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
            track_y_points: vec![],
            track_nodes,
            ground_nodes,
            move_blocks,
            track_blocks,
            creepers,
            object_link_type: vec![],
            objects,
            ground,
            ice,
            tracks,
            tile_loc: vec![],
            pipe_loc: vec![],
            ground_loc: vec![],
            obj_loc_data: vec![],
        })
    }
}

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
