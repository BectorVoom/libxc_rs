//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2205/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2205<F: Float>(t1530: F, t584: F, t86730: F, t25372: F, t5397: F, t868: F, t28248: F, t81547: F, t5660: F, t606: F, t17109: F, t25: F) -> (F, F, F, F, F) {
    let t98069 = t86730 * t584 * t1530;
    let t98071 = F::new(2.0) * t25372 * t98069;
    let t98075 = t5397 * t868;
    let t98079 = t81547 * t28248;
    let t98082 = t606 * t5660;
    let t98086 = t25 * t17109;
    (t98071, t98075, t98079, t98082, t98086)
}
