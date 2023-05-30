use mahf_tsplib::Instances;
use strum::{IntoEnumIterator};

#[test]
fn load_all_instances() {
    for instance in Instances::iter() {
        let _tsp = instance.load();
    }
}
