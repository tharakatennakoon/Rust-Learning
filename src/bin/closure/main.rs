mod basic;
mod as_input_parameter;

fn main() {
    basic::basic();
    basic::capturing();
    basic::closure_clone();

    as_input_parameter::test_closure();

}
