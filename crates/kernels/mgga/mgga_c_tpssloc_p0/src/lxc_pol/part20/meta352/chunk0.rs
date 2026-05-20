//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1661/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1661<F: Float>(t12240: F, t6977: F, t3851: F, t3901: F, t1337: F, t562: F, t3792: F, t550: F, t12177: F) -> (F, F, F, F, F, F, F) {
    let t12241 = t6977 * t12240;
    let t12244 = t3901 * t3851;
    let t12247 = t1337 * t1337;
    let t12248 = F::new(1.0) / t12247;
    let t12249 = t12248 * t562;
    let t12250 = t3792 * t550;
    let t12251 = t12177 * t12250;
    (t12241, t12244, t12247, t12248, t12249, t12250, t12251)
}
