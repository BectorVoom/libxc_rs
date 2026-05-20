//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1914/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1914<F: Float>(t1992: F, t54918: F, t550: F, t6976: F, t22690: F, t552: F, t26447: F, t90607: F, t22751: F, t26397: F, t22892: F, t22893: F, t26396: F) -> (F, F, F, F, F) {
    let t90785 = t1992 * t6976 * t54918 * t550;
    let t90787 = t22690 * t552;
    let t90789 = t90607 * t90787 * t26447;
    let t90791 = t22751 * t26397;
    let t90794 = t22892 * t22893 * t26396;
    (t90785, t90787, t90789, t90791, t90794)
}
