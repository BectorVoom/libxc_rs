//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1950/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1950<F: Float>(t5371: F, t7467: F, t5456: F, t576: F, t1873: F, t1458: F, t3941: F, t5493: F, t1401: F, t28017: F, t1409: F, t22510: F, t24498: F, t27356: F, t5392: F, t5398: F, t5415: F, t56: F, t7251: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28892 = F::new(27.0) * t5371 * t7467;
    let t28893 = t576 * t5456;
    let t28895 = F::new(27.0) * t28893 * t1873;
    let t28896 = t7467 * t1458;
    let t28898 = F::new(54.0) * t3941 * t28896;
    let t28899 = t1873 * t5493;
    let t28901 = F::new(27.0) * t3941 * t28899;
    let t28903 = F::new(0.135e2) * t1401 * t28017;
    let t29473 = F::new(88.0) / F::new(9.0) * t5415 * t56 + F::new(40.0) / F::new(9.0) * t27356 * t1409 + F::new(5.0) / F::new(18.0) * t24498 * t5392 - F::new(5.0) / F::new(6.0) * t7251 * t5398 - t22510;
    (t28892, t28893, t28895, t28896, t28898, t28899, t28901, t28903, t29473)
}
