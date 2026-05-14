//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1139/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1139<F: Float>(t112820: F, t23083: F, t30706: F, t23094: F, t30703: F, t23103: F, t794: F, t8339: F, t30719: F, t808: F, t8344: F, t226: F, t235: F, t2690: F, t23139: F, t23171: F, t23228: F, t8335: F) -> (F, F, F, F, F, F, F, F) {
    let t112821 = 7.0 / 288.0 * t112820;
    let t112829 = t23083 * t30706;
    let t112830 = 0.11304371706359309439e-1 * t112829;
    let t112834 = t23094 * t30703;
    let t112840 = t23103 * t794 * t8339;
    let t112846 = t808 * t30719 * t8344;
    let t112847 = 7.0 / 1152.0 * t112846;
    let t112850 = t226 * t235 * t2690 * t8344;
    let t112855 = t23139 * t8339;
    let t112863 = 0.16449340668482264365e-1 * t23171 * t23228 * t8335;
    (t112821, t112830, t112834, t112840, t112847, t112850, t112855, t112863)
}
