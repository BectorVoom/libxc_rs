//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1043/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1043<F: Float>(t30: F, t259: F, t379: F, t10937: F, t11219: F, t11796: F, t10353: F, t10947: F, t10948: F, t10950: F, t1288: F, t1289: F, t1402: F, t1490: F, t1991: F, t1992: F, t2445: F, t2818: F, t3431: F, t3735: F, t381: F, t4028: F, t45: F, t580: F, t581: F, t999: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F,) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t11798 = piecewise3(t380, t11219 + t11796, t10937);
    let t11810 = piecewise3(t120, t10937 * t30 / 2.0 + t3735 * t580 + t1402 * t1991 / 2.0 + t2445 * t1288 / 2.0 + t10947 + t10948 - t10950, t11798 * t45 / 2.0 + t4028 * t581 + t1490 * t1992 / 2.0 + t2818 * t1289 / 2.0 + t999 * t3431 + t381 * t10353 / 2.0);
    (t11810,)
}
