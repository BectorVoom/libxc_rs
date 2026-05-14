//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1028/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1028<F: Float>(t30623: F, t81651: F, t82074: F, t2717: F, t6662: F, t30642: F, t6562: F, t794: F, t1902: F, t213: F, t225: F, t30745: F, t23030: F, t30638: F, t212: F, t23171: F, t6554: F) -> (F, F, F, F, F, F, F) {
    let t112867 = t81651 * t82074 * t30623;
    let t112873 = t2717 * t6662;
    let t112892 = t6562 * t794 * t30642;
    let t112899 = t213 * t1902 * t225;
    let t112908 = t30745 * t225;
    let t112936 = 0.52089578783527170489e-1 * t23030 * t30638;
    let t112942 = 0.16449340668482264365e-1 * t23171 * t212 * t1902 * t6554;
    (t112867, t112873, t112892, t112899, t112908, t112936, t112942)
}
