//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 753/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk753<F: Float>(t533: F, t7752: F, t1390: F, t1983: F, t2019: F, t5161: F, t1873: F, t5371: F, t1458: F, t3941: F, t1401: F, t7467: F, t1409: F, t1419: F, t56: F, t6503: F, t7251: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7753 = t533 * t7752;
    let t7754 = t7753 * t1390;
    let t7755 = t1983 * t7754;
    let t7756 = t2019 * t5161;
    let t7757 = t1983 * t7756;
    let t7768 = 0.135e2 * t5371 * t1873;
    let t7769 = t1873 * t1458;
    let t7771 = 27.0 * t3941 * t7769;
    let t7773 = 0.135e2 * t1401 * t7467;
    let t7973 = -8.0 / 3.0 * t1419 * t56 - 5.0 / 6.0 * t7251 * t1409 + t6503;
    (t7753, t7754, t7755, t7756, t7757, t7768, t7769, t7771, t7773, t7973)
}
