//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 904/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk904<F: Float>(t3451: F, t4919: F, t3295: F, t3464: F, t4770: F, t4773: F, t4776: F, t4779: F, t457: F, t460: F, t974: F, t1184: F, t1714: F) -> (F, F, F, F, F, F) {
    let t4920 = t4919 * t3451;
    let t4928 = -t3464 + t3295 / F::cast_from(9.0_f64) + t4770 / F::cast_from(9.0_f64) + t4773 / F::cast_from(18.0_f64) - t4776 / F::cast_from(3.0_f64) - t4779 / F::cast_from(6.0_f64);
    let t4929 = t457 * t4928;
    let t4930 = t4929 * t460;
    let t4931 = t974 * t4930;
    let t4934 = t974 * t457;
    let t4935 = t1714 * t1184;
    (t4920, t4928, t4930, t4931, t4934, t4935)
}
