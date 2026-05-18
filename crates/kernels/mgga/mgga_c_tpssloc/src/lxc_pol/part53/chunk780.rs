//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 780/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk780<F: Float>(t1484: F, t258: F, t776: F, t23270: F, t25038: F, t1527: F, t2717: F, t865: F, t1888: F, t6547: F, t7485: F, t857: F) -> (F, F, F, F, F, F) {
    let t25039 = t258 * t1484;
    let t25040 = t25039 * t776;
    let t25041 = t23270 * t25040;
    let t25042 = t25038 * t25041;
    let t25044 = t2717 * t1527;
    let t25045 = t25044 * t865;
    let t25046 = t23270 * t25045;
    let t25047 = t1888 * t25046;
    let t25049 = t6547 * t7485;
    let t25053 = t857 * t1527;
    (t25040, t25042, t25045, t25047, t25049, t25053)
}
