//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 773/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk773<F: Float>(t232: F, t2553: F, t2645: F, t2646: F, t2614: F, t838: F, t2693: F, t809: F, t225: F, t9584: F, t237: F, t597: F, t61: F) -> (F, F, F, F, F, F) {
    let t10007 = t232 * t2553;
    let t10009 = t2645 * t2646 * t10007;
    let t10012 = t2614 * t838;
    let t10014 = t809 * t2693;
    let t10016 = t9584 * t225;
    let t10017 = t10016 * t237;
    let t10021 = F::new(1.0) / t61 / t597;
    (t10009, t10012, t10014, t10016, t10017, t10021)
}
