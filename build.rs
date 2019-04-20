fn main() {
    println!("cargo:rustc-cfg=println");
    println!("rustc-link-search=/home/aleksanb/Projects/fourkay/");
    println!("rustc-link-lib=static=4klang");
}
