fn main() {
    println!("cargo:rustc-link-arg=/NODEFAULTLIB");
    println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");

    // No safe exception handlers
    println!("cargo:rustc-link-arg=/SAFESEH:NO");
    println!("cargo:rustc-link-arg=/DYNAMICBASE:NO");
    println!("cargo:rustc-link-arg=/ENTRY:WinMainCRTStartup");
    println!("cargo:rustc-link-arg=/LTCG");
    // println!("cargo:rustc-link-arg=/OPT:NOWIN98");
    println!("cargo:rustc-link-arg=msvcrt.lib");
}
