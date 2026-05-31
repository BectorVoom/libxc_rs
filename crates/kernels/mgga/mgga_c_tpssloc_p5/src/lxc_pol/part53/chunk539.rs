//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 539/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk539<F: Float>(t1147: F, t1687: F, t1155: F, t1695: F, t3238: F, t3295: F, t3383: F, t3390: F, t4721: F, t4726: F, t4731: F, t4735: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F, t4773: F, t4776: F, t4779: F) -> (F, F, F) {
    let t4835 = t1687 * t1147;
    let t4840 = t1695 * t1155;
    let t4857 = -F::cast_from(0.1294625e1_f64) * t4749 + F::cast_from(0.258925e1_f64) * t4757 + t3383 - F::cast_from(0.10064166666666666667e0_f64) * t3238 - F::cast_from(0.10064166666666666667e0_f64) * t4721 - F::cast_from(0.20128333333333333333e0_f64) * t4726 + F::cast_from(0.60385e0_f64) * t4731 + F::cast_from(0.301925e0_f64) * t4735 + F::cast_from(0.82524375e-1_f64) * t4765 + F::cast_from(0.16504875e0_f64) * t4767 + t3390 - F::cast_from(0.5519e-1_f64) * t3295 - F::cast_from(0.5519e-1_f64) * t4770 - F::cast_from(0.27595e-1_f64) * t4773 + F::cast_from(0.16557e0_f64) * t4776 + F::cast_from(0.82785e-1_f64) * t4779;
    (t4835, t4840, t4857)
}
