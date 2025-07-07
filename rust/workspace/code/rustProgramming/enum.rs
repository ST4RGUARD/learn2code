
enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main() {
    let disk_type = DiskType::SSD;

    // can't compare
    // if disk_type == DiskType::SSD {
    //     println!("disk type is ssd");
    // } else {
    //     println!("disk type is hdd");
    // }

    match disk_type {
        DiskType::SSD => println!("disk type is ssd"),
        DiskType::HDD => println!("disk type is hdd"),
    }

    let disk_size = DiskSize::GB(128);
    println!("{:?}", disk_size);

}
