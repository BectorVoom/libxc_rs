//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1163/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1163<F: Float>(t1014: F, t42340: F, t42341: F, t23508: F, t360: F, t3127: F, t3131: F, t10474: F, t10482: F, t43288: F, t43292: F, t10163: F, t386: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43503 = t42340 * t42341 * t1014;
    let t43505 = t23508 * t360;
    let t43515 = t42340 * t42341 * t3127;
    let t43516 = t23508 * t3131;
    let t43553 = t42340 * t42341 * t10474;
    let t43554 = t23508 * t10482;
    let t43576 = t42340 * t42341 * t43288;
    let t43577 = t23508 * t43292;
    let t43603 = F::new(1.0) / t10163 / t386;
    (t43503, t43505, t43515, t43516, t43553, t43554, t43576, t43577, t43603)
}
