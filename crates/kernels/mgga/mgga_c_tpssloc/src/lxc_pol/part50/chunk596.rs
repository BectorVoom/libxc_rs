//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 596/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk596<F: Float>(t3815: F, t1788: F, t588: F, t592: F, t3829: F, t3833: F, t2426: F, t2486: F, t3819: F, t3821: F, t3825: F, t3827: F, t3832: F, t5169: F, t225: F, t5262: F) -> (F, F, F, F, F, F) {
    let t5263 = 0.18311447306006545054e-3 * t3815;
    let t5264 = t588 * t1788;
    let t5265 = 4.0 * t5264;
    let t5266 = t592 * t1788;
    let t5267 = 4.0 * t5266;
    let t5268 = 4.0 * t3829;
    let t5269 = 4.0 * t3833;
    let t5270 = t5169 - t5263 - t2426 + t3819 - t3821 + t3825 + t5265 - t5267 + t3827 - t5268 - t2486 - t3832 - t5269;
    let t5272 = (t5262 + t5270) * t225;
    (t5263, t5265, t5267, t5268, t5269, t5272)
}
