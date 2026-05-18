//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 749/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk749<F: Float>(t533: F, t6995: F, t1390: F, t1983: F, t1388: F, t3701: F, t2019: F, t1873: F, t3938: F, t671: F, t3941: F, t1401: F, t6534: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6996 = t533 * t6995;
    let t6997 = t6996 * t1390;
    let t6998 = t1983 * t6997;
    let t6999 = t3701 * t1388;
    let t7000 = t2019 * t6999;
    let t7001 = t1983 * t7000;
    let t7014 = F::new(0.135e2) * t3938 * t1873;
    let t7015 = t1873 * t671;
    let t7017 = F::new(27.0) * t3941 * t7015;
    let t7019 = F::new(0.135e2) * t1401 * t6534;
    (t6996, t6997, t6998, t6999, t7000, t7001, t7014, t7015, t7017, t7019)
}
