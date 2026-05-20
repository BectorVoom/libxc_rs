//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1201/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1201<F: Float>(t16132: F, t1825: F, t1352: F, t19743: F, t19660: F, t118: F, t6330: F, t794: F, t12202: F, t19631: F, t210: F, t214: F) -> (F, F, F, F, F) {
    let t19756 = t16132 * t1825;
    let t19761 = t19743 * t1352;
    let t19763 = t19660 * t1352;
    let t19767 = t118 * t794 * t6330;
    let t19768 = t12202 * t19767;
    let t19771 = t210 * t214 * t19631;
    (t19756, t19761, t19763, t19768, t19771)
}
