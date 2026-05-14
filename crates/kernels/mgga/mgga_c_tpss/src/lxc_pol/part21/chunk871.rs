//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 871/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk871<F: Float>(t5: F, t1675: F, t1680: F, t5487: F, t6073: F, t6077: F, t6080: F, t6087: F, t6091: F, t117: F, t1689: F, t3493: F, t1338: F, t94: F) -> (F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t6095 = piecewise3(t8, 0.0, -t6073 * t1680 / 6.0 + 5.0 / 6.0 * t5487 * t6077 + t6080 * t1680 / 3.0 - t1675 * t6087 / 6.0 - t1675 * t6091 / 6.0);
    let t6096 = t6095 * t117;
    let t6102 = 2.0 * t3493 * t1689;
    let t6103 = t94 * t1338;
    (t6095, t6096, t6102, t6103)
}
