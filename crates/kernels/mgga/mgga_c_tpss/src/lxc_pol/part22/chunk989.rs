//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 989/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk989<F: Float>(t3621: F, t750: F, t762: F, t1368: F, t2133: F, t2158: F, t339: F, t790: F, t3632: F, t10623: F, t10630: F, t10632: F, t10635: F, t10638: F, t10642: F, t2147: F, t761: F, t797: F, t8127: F, t8131: F, t8133: F, t8168: F, t8171: F) -> F {
    let t10644 = t762 * t3621 * t750;
    let t10648 = t762 * t1368 * t2133;
    let t10652 = t339 * t2158 * t790;
    let t10654 = F::new(7.0) / F::new(1152.0) * t10652 * t3632;
    let t10656 = -t797 * t10623 / F::new(768.0) - F::new(35.0) / F::new(1152.0) * t8127 - F::new(119.0) / F::new(1728.0) * t8131 + F::new(7.0) / F::new(1152.0) * t8133 + t10630 - t761 * t10632 / F::new(48.0) - F::new(35.0) / F::new(216.0) * t10635 - t8171 * t10638 / F::new(4.0) - t10642 + t2147 * t10644 / F::new(8.0) + t2147 * t10648 / F::new(16.0) - t10654 - F::new(7.0) / F::new(48.0) * t8168;
    t10656
}
