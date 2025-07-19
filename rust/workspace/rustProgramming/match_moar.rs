enum WineGrapes {
    CabernetFranc,
    Tannat,
    Merlot,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes{
        WineGrapes::CabernetFranc => println!("this is a cab franc wine"),
        _ => println!("not cab franc"),
        // //WineGrapes::Tannat => println!("tannat"),
        // WineGrapes::Merlot => println!("merlot"),
    }
}
fn main() {
    taste_wine(WineGrapes::CabernetFranc);
}
