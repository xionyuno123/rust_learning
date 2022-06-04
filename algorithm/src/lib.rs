#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


mod implement;

pub mod divide_and_conquer {
    //! Divide-and-Conquer
    //! - **Divide** the problem into a number of subproblems that are smaller instances of the same problem.
    //! - **Conquer** the subproblems by solving them recursively. If the subproblem sizes are small enough, however
    //! ,just solve the subproblems in a straightforward manner.
    //! - **Combine** the solutions to the subproblems into the solution for the original problem.

    
    pub use super::implement::part1::max_subarray::max_subarray_with_divide_and_conquer as max_subarray;
}


pub mod dynamic_programming {
    
    pub use super::implement::part1::max_subarray::max_subarray_with_dp as max_subarray;
}


pub mod compare_sort {
    pub use super::implement::part1::compare_sort::*;
}