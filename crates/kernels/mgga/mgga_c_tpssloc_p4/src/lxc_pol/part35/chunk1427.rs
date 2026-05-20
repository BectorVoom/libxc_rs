//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1427/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1427<F: Float>(t22574: F, t28830: F, t33136: F, t106956: F, t1874: F, t1873: F, t20347: F, t3941: F, t1458: F, t28017: F, t5493: F, t7467: F) -> (F, F, F, F, F) {
    let t107533 = F::new(18.0) * t22574 * t33136 * t28830;
    let t107539 = F::new(6.0) * t106956 * t1874;
    let t107552 = F::new(27.0) * t3941 * t1873 * t20347;
    let t107555 = F::new(81.0) * t3941 * t28017 * t1458;
    let t107558 = F::new(81.0) * t3941 * t7467 * t5493;
    (t107533, t107539, t107552, t107555, t107558)
}
