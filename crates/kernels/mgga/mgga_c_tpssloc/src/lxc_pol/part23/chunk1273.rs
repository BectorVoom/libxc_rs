//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1273/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1273<F: Float>(t25: F, t265: F, t394: F, t76559: F, t76666: F, t77918: F, t77920: F, t77929: F, t1408: F, t1409: F, t1534: F, t1642: F, t20216: F, t20217: F, t21076: F, t21703: F, t396: F, t40: F, t5397: F, t5398: F, t5669: F, t5955: F, t75911: F, t75912: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F,) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t77932 = piecewise3(t395, t76666 + t77918 + t77920 + t77929, t76559);
    let t77944 = piecewise3(t115, t76559 * t25 / 2.0 + 2.0 * t21076 * t1408 + 3.0 * t5669 * t5397 + 2.0 * t1534 * t20216 + t265 * t75911 / 2.0, t77932 * t40 / 2.0 + 2.0 * t21703 * t1409 + 3.0 * t5955 * t5398 + 2.0 * t1642 * t20217 + t396 * t75912 / 2.0);
    (t77944,)
}
