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
  src: i64,
  dst_diff: i64,
  rng: i64
}

struct CustomMap {
  map_list: Vec<Mapping>
}

impl CustomMap {
  pub fn get_mapped_value(&self, val: i64) -> i64 {
    for m in &self.map_list {
      if val >= m.src && val <= m.src+m.rng {
        return val+m.dst_diff;
      }
    } 
    val
  }

  pub fn add_map_entry(&mut self, dst: i64, src: i64, rng: i64) {
    self.map_list.push(Mapping { src: src, dst_diff: dst - src, rng: rng });
  }
}

struct MapCollection {
  seeds: Vec<i64>,
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
  fn get_seeds_list (line: &str) -> Vec<i64> {
    let seeds_list_str = line.strip_prefix("seeds: ").unwrap();
    let mut seed_list: Vec<i64> = Vec::new();
    for seed_str in seeds_list_str.split(' ').into_iter() {
      println!("Try to parse: {}", seed_str);
      seed_list.push(seed_str.parse::<i64>().unwrap())
    }
    seed_list
  }

  fn add_to_map(&mut self, map_type: &MapType, line: &str) {
    let value_list: Vec<&str> = line.split(" ").collect();
    if value_list.len() != 3 {
      return;
    }
    let map: &mut CustomMap;
    match map_type {
        MapType::Undefined => return,
        MapType::Soil => map = &mut self.soil_map,
        MapType::Fertilizer => map = &mut self.fertilizer_map,
        MapType::Water => map = &mut self.water_map,
        MapType::Light => map = &mut self.light_map,
        MapType::Temp => map = &mut self.temp_map,
        MapType::Humidity => map = &mut self.humidity_map,
        MapType::Location => map = &mut self.location_map
    }
    map.add_map_entry(value_list[0].parse::<i64>().unwrap(), value_list[1].parse::<i64>().unwrap(), value_list[2].parse::<i64>().unwrap());
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
  
  fn find_location(&self, seed: i64) -> i64 {
    let soil_val = self.soil_map.get_mapped_value(seed);
    let fertilizer_val = self.fertilizer_map.get_mapped_value(soil_val);
    let water_val = self.water_map.get_mapped_value(fertilizer_val);
    let light_val = self.light_map.get_mapped_value(water_val);
    let temp_val = self.temp_map.get_mapped_value(light_val);
    let humidity_val = self.humidity_map.get_mapped_value(temp_val);
    self.location_map.get_mapped_value(humidity_val)
  }

  pub fn find_closest_location(&self) -> i64{
    let mut closest_location = i64::MAX;
    for seed in self.seeds.iter() {
      let location = self.find_location(*seed);
      if location < closest_location {
        closest_location = location;
      }
    }
    closest_location
  }
}

fn main() {
    let input = file_to_str("./input.txt");
    let mut map_collection = MapCollection{seeds: Vec::new(), 
      fertilizer_map: CustomMap { map_list: Vec::new() }, soil_map: CustomMap { map_list: Vec::new() }, 
      water_map: CustomMap { map_list: Vec::new() }, light_map: CustomMap { map_list: Vec::new() }, 
      temp_map: CustomMap { map_list: Vec::new() }, humidity_map: CustomMap { map_list: Vec::new() }, 
      location_map: CustomMap { map_list: Vec::new() }};
    map_collection.build_maps_from_str(&input);
    let closest_seed = map_collection.find_closest_location();
    println!("Closest location: {}", closest_seed);
}
