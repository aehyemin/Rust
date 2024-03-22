// other collection types
// HashMap, BTreeMap
// Key, Value
// Key : String
// Value :Vec<String>
// land : 나라, 국가 
// HashMap<Stringm Vec<String>>


use std::collections::HashMap;

struct city {
    name: String,
    population : HashMap<u32, u32> // year + population
}

fn main(){
    let mut tallinn = city {
        name : "Tallinn".to_string(),
        population: HashMap::new()
    };
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24000);
    tallinn.population.insert(2020, 437619);

    for (year, population) in tallinn.population{
        println!("In the year {} the population was {}", year, population);
    }

}

/* 
use std::collections::BTreeMap;

struct City {
    name: String,
    population : BTreeMap<u32, u32> // year + population
}

fn main(){
    let mut tallinn = City {
        name : "Tallinn".to_string(),
        population: BTreeMap::new()
    };
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24000);
    tallinn.population.insert(2020, 437619);

    for (year, population) in tallinn.population{
        println!("In the year {} the population was {}", year, population);
    }

}
*/