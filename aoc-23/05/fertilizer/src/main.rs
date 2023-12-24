fn file_to_str(filename: &str) -> String {
  use std::fs::File;
  use std::io::Read;
  use std::io::BufReader;
  let file = File::open(filename).unwrap();
  let mut buf_reader = BufReader::new(file);
  let mut content = String::new();
  buf_reader.read_to_string(&mut content).unwrap();
  content
}

struct Mapping {
  src: i32,
  dst_diff: i32,
  rng: i32
}

struct CustomMap {
  map_list: Vec<Mapping>
}

impl CustomMap {
  pub fn get_mapped_value(&self, val: i32) -> i32 {
    for m in &self.map_list {
      if val >= m.src && val <= m.src+m.rng {
        return val+m.dst_diff;
      }
    } 
    val
  }

  pub fn add_map_entry(&mut self, dst: i32, src: i32, rng: i32) {
    self.map_list.push(Mapping { src: src, dst_diff: dst - src, rng: rng });
  }
}

struct MapCollection {
  seeds: Vec<i32>,
  soil_map: CustomMap,
  fertilizer_map: CustomMap,
  water_map: CustomMap, 
  light_map: CustomMap,
  temp_map: CustomMap, 
  humidity_map: CustomMap,
  location_map: CustomMap
}

enum MapType {
  Soil, Fertilizer, Water, Light, Temp, Humidity, Location, Undefined
}

impl MapCollection {
  fn get_seeds_list (line: &str) -> Vec<i32> {
    let seeds_list_str = line.strip_prefix("seeds: ").unwrap();
    let mut seed_list: Vec<i32> = Vec::new();
    for seed_str in seeds_list_str.split(' ').into_iter() {
      seed_list.push(seed_str.parse::<i32>().unwrap())
    }
    seed_list
  }

  fn add_to_map(&mut self, mapType: &MapType, line: &str) {
    let value_list: Vec<&str> = line.split(" ").collect();
    if value_list.len() != 3 {
      println!("Line does not contain 3 values! {}", line);
      return;
    }
    let map: &mut CustomMap;
    match mapType {
        MapType::Undefined => return,
        MapType::Soil => map = &mut self.soil_map,
        MapType::Fertilizer => map = &mut self.fertilizer_map,
        MapType::Water => map = &mut self.water_map,
        MapType::Light => map = &mut self.light_map,
        MapType::Temp => map = &mut self.temp_map,
        MapType::Humidity => map = &mut self.humidity_map,
        MapType::Location => map = &mut self.location_map
    }
    map.add_map_entry(value_list[0].parse::<i32>().unwrap(), value_list[1].parse::<i32>().unwrap(), value_list[2].parse::<i32>().unwrap());
  }


  pub fn build_maps_from_str(&mut self, input: &str) {
    let mut current_type = MapType::Undefined;
    for line in input.lines() {
      if line.starts_with("seeds: ") {
        self.seeds = Self::get_seeds_list(line);
      } else if line.starts_with("seed-to-soil map:") {
        current_type = MapType::Soil;
      } else if line.starts_with("soil-to-fertilizer map:") {
        current_type = MapType::Fertilizer;
      } else if line.starts_with("fertilizer-to-water map:") {
        current_type = MapType::Water;
      } else if line.starts_with("water-to-light map:") {
        current_type = MapType::Light;
      } else if line.starts_with("light-to-temperature map:") {
        current_type = MapType::Temp;
      } else if line.starts_with("temperature-to-humidity map:") {
        current_type = MapType::Humidity;
      } else if line.starts_with("humidity-to-location map:") {
        current_type = MapType::Location;
      }
      self.add_to_map(&current_type, line);
    }
  }
}

fn main() {
    let input = file_to_str("./input.txt");
    let map_collection = MapCollection{seeds: Vec::new(), 
      fertilizer_map: CustomMap { map_list: Vec::new() }, soil_map: CustomMap { map_list: Vec::new() }, 
      water_map: CustomMap { map_list: Vec::new() }, light_map: CustomMap { map_list: Vec::new() }, 
      temp_map: CustomMap { map_list: Vec::new() }, humidity_map: CustomMap { map_list: Vec::new() }, 
      location_map: CustomMap { map_list: Vec::new() }};
}
