#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Loire,
    Rhone,
    NapaValley,
    Tuscany,
}

struct Wine {
    name: String,
    region: WineRegions,
}

fn supported_regions(wine: WineRegions) {
    match wine {
        WineRegions::Bordeaux => println!("Bordeaux is supported!"),
        WineRegions::Burgundy => println!("Burgundy is supported!"),
        WineRegions::Champagne => println!("Champagne is supported!"),
        WineRegions::Loire => println!("Loire is supported!"),
        WineRegions::Rhone => println!("Rhone is supported!"),
        WineRegions::NapaValley => println!("Napa Valley is supported!"),
        WineRegions::Tuscany => println!("Tuscany is supported!"),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    // supported_regions(wine1);
    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);

    supported_regions(wine1.region);
    supported_regions(wine2.region);
    supported_regions(WineRegions::NapaValley);
}