#[derive(Debug)]

enum WineRegions {
    Bordeaux,
    Burgundy,
    Tuscany,
    Rock,
}

struct Wine {
    name: String,
    region: WineRegions,
}

fn supported_regions(w: WineRegions){
    match w {
        WineRegions::Rock => println!("rock is listed"),
        _ => println!("{:?} is not supported", w),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("chateau margaux"),
        region: WineRegions::Bordeaux,
    };
    let wine2 = Wine {
        name: String::from("barolo"),
        region: WineRegions::Tuscany,
    };

    println!("wine1 {} from {:?} ", wine1.name, wine1.region);
    println!("wine2 {} from {:?} ", wine2.name, wine2.region);

    supported_regions(WineRegions::Burgundy);
}
