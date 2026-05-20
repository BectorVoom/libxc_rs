//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2175/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2175<F: Float>(t111: F, t19449: F, t19681: F, t2528: F, t172: F, t19572: F, t763: F, t2535: F, t2371: F, t19575: F, t592: F, t1390: F, t20063: F) -> (F, F, F, F, F, F, F) {
    let t55943 = t19449 * t111;
    let t56099 = t19681 * t2528;
    let t56102 = t19572 * t172 * t763;
    let t56104 = t19681 * t2535;
    let t56168 = t19681 * t2371;
    let t56185 = t592 * t19575;
    let t56358 = t20063 * t1390;
    (t55943, t56099, t56102, t56104, t56168, t56185, t56358)
}
