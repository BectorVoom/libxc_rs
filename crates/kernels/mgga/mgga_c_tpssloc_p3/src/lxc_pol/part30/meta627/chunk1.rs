//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2031/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2031<F: Float>(t87300: F, t1496: F, t81942: F, t7497: F, t81933: F, t25098: F, t81835: F, t13176: F, t6620: F, t25097: F, t81782: F, t81783: F) -> (F, F, F, F, F, F) {
    let t87301 = F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t87300;
    let t87304 = t81942 * t1496;
    let t87306 = t81933 * t7497;
    let t87308 = t81835 * t25098;
    let t87321 = t13176 * t6620;
    let t87328 = t81782 * t81783 * t25097;
    (t87301, t87304, t87306, t87308, t87321, t87328)
}
