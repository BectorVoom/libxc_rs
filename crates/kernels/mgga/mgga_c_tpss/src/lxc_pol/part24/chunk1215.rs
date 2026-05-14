//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1215/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1215<F: Float>(t5: F, t21169: F, t117: F, t4525: F, t6274: F, t1760: F, t13565: F, t1689: F, t1321: F, t1338: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t21170 = piecewise3(t8, 0.0, t21169);
    let t21171 = t21170 * t117;
    let t21175 = t6274 * t4525;
    let t21177 = 2.0 * t1760 * t21175;
    let t21179 = 2.0 * t13565 * t1689;
    let t21180 = t1321 * t1338;
    (t21170, t21171, t21175, t21177, t21179, t21180)
}
