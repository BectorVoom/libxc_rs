//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 933/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk933<F: Float>(t12020: F, t2091: F, t225: F, t7910: F, t7919: F, t1824: F, t2085: F, t1338: F, t7918: F, t111: F, t7786: F) -> (F, F, F, F, F, F) {
    let t26989 = t12020 * t2091;
    let t27009 = t7910 * t225;
    let t27068 = t7919 * t225;
    let t27074 = t2085 * t1824;
    let t27097 = t1338 * t7918;
    let t27188 = t7786 * t111;
    (t26989, t27009, t27068, t27074, t27097, t27188)
}
