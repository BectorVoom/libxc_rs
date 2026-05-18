//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1116/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1116<F: Float>(t4677: F, t4684: F, t14506: F, t3185: F, t1932: F, t3120: F, t360: F, t1629: F, t1625: F, t3040: F, t3201: F, t6739: F) -> (F, F, F, F, F, F) {
    let t14615 = t4677 * t4684;
    let t14618 = t14506 * t3185;
    let t14622 = t1932 * t3120 * t360;
    let t14623 = t1629 * t14622;
    let t14626 = t1625 * t3040;
    let t14627 = t14626 * t3201;
    let t14630 = t6739 * t3040 * t360;
    (t14615, t14618, t14623, t14626, t14627, t14630)
}
