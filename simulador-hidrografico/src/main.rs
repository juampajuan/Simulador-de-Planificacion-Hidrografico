use gdal::version::VersionInfo;

fn main() {
    println!("{}", gdal::version_info("VERSION_NUM"));
    // 3060200
    println!("{}", VersionInfo::release_name());
    // 3.6.2
}
