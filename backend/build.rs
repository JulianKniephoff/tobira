use vergen::vergen;

fn main() {
    //
    let mut config = vergen::Config::default();
    *config.git_mut().semver_dirty_mut() = Some(", dirty");
    *config.git_mut().semver_kind_mut() = vergen::SemverKind::Lightweight;
    *config.git_mut().sha_kind_mut() = vergen::ShaKind::Short;
    vergen(config).expect("setting build info failed");
    // TODO: Only rerun when necessary?
}
