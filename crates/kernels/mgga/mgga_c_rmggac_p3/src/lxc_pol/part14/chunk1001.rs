//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1001/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1001<F: Float>(t27091: F, t40901: F, t40487: F, t5148: F, t39059: F, t5271: F, t39063: F, t5259: F, t2402: F, t839: F, t2367: F, t321: F) -> (F, F, F, F, F, F) {
    let t41077 = t27091 * t40901;
    let t41079 = t5148 * t40487;
    let t41084 = t5271 * t39059;
    let t41086 = t5259 * t39063;
    let t41088 = t2402 * t839;
    let t41091 = t2367 * t321;
    (t41077, t41079, t41084, t41086, t41088, t41091)
}
