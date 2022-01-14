mod random_info;
use random_info::*;

#[allow(dead_code)]
struct PrinceData {
    some_int: i32,
    some_float: f64,
    some_bool: bool,
    random: RandomInfo,
}

impl RandomInfo {
    pub fn is_larger(&self, compare_to: i64) -> bool {
        self.some_int > compare_to
    }
}

impl SomeTrait for PrinceData {
    fn is_valid(&self) -> bool {
        true
    }
}

fn print_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Yay!")
    }
}

impl Default for PrinceData {
    fn default() -> Self {
        Self {
            some_bool: true,
            some_float: 10.3,
            some_int: 80,
            random: RandomInfo::new(true),
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let mut random_info_var = RandomInfo {
        call_count: 0,
        some_bool: false,
        some_int: 1,
    };
    let is_this_smaller = random_info_var.is_smaller(9);
    let is_this_larger = random_info_var.is_larger(20);
    let is_valid = random_info_var.is_valid();

    // let prince_var = PrinceData {
    //     some_bool: true,
    //     some_float: 10.3,
    //     some_int: 80,
    //     random: RandomInfo::new(true),
    // };
    // prince_var.some_int = 100;

    // let prince_var_2 = PrinceData {
    //     some_int: 200,
    //     ..prince_var
    // };

    let prince_var = PrinceData::default();

    print_if_is_valid(&random_info_var);
    print_if_is_valid(&prince_var);
}
