//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 501/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk501<F: Float>(t1178: F, t5398: F, t1177: F, t3464: F, t4770: F, t6012: F, t6015: F, t6018: F, t457: F, t460: F, t974: F, t1714: F) -> (F, F, F, F, F) {
    let t6130 = t1178 * t5398;
    let t6131 = t1177 * t6130;
    let t6138 = -t3464 + F::new(2.0) / F::new(9.0) * t4770 + t6012 / F::new(18.0) - t6015 / F::new(3.0) - t6018 / F::new(6.0);
    let t6139 = t457 * t6138;
    let t6140 = t6139 * t460;
    let t6141 = t974 * t6140;
    let t6144 = t1714 * t1714;
    (t6131, t6138, t6140, t6141, t6144)
}
