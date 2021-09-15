///module that contains all the solutions to project eulers problems.
///
/// # Description
///
/// inside this module, there is a method/function for each solved problem, in the form of problem_XXX, where XXX is the corresponding number from the project euler site.
/// There are some functions that are usefull to multiple problems, these should eventually be put in some sort of lib.rs library file.
mod problems;


///used only to select a single problem from the problems module
fn main() {
    problems::problem_021();

}
