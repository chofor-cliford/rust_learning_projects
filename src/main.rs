enum WineGrapes {
    Merlot,
    CabernetSauvignon,
    PinotNoir,
    Chardonnay,
    SauvignonBlanc,
    Riesling,
    Syrah,
    Zinfandel,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::Merlot => println!("Merlot: Fruity, medium to full-bodied, dry red wine."),
        WineGrapes::CabernetSauvignon => println!("Cabernet Sauvignon: Full-bodied, dry red wine."),
        WineGrapes::PinotNoir => println!("Pinot Noir: Light to medium-bodied, dry red wine."),
        WineGrapes::Chardonnay => println!("Chardonnay: Full-bodied, dry white wine."),
        WineGrapes::SauvignonBlanc => println!("Sauvignon Blanc: Dry white wine."),
        WineGrapes::Riesling => println!("Riesling: Sweet white wine."),
        WineGrapes::Syrah => println!("Syrah: Full-bodied, dry red wine."),
        WineGrapes::Zinfandel => println!("Zinfandel: Medium to full-bodied, dry red wine."),
    }
}

fn main() {
    taste_wine(WineGrapes::Merlot);
    taste_wine(WineGrapes::Chardonnay);
    taste_wine(WineGrapes::Riesling);
    taste_wine(WineGrapes::Syrah);
    taste_wine(WineGrapes::CabernetSauvignon);
    taste_wine(WineGrapes::PinotNoir);
    taste_wine(WineGrapes::Zinfandel);
}
 