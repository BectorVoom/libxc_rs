//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1177/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1177<F: Float>(t225: F, t32878: F, t112680: F, t112686: F, t112702: F, t30713: F, t4166: F, t30716: F, t112797: F, t32844: F, t13242: F, t232: F, t30714: F, t4180: F) -> (F, F, F, F, F, F, F) {
    let t118510 = t32878 * t225;
    let t118518 = F::new(0.76763589786250567036e-1) * t112680;
    let t118523 = F::new(0.76763589786250567036e-1) * t112686;
    let t118526 = F::new(0.16449340668482264365e-1) * t112702;
    let t118532 = t4166 * t30713;
    let t118533 = t118532 * t30716;
    let t118535 = t112797 * t32844;
    let t118539 = t30714 * t4180 * t13242 * t232;
    (t118510, t118518, t118523, t118526, t118533, t118535, t118539)
}
