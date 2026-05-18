//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 645/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk645<F: Float>(t136: F, t4775: F, t1113: F, t4733: F, t3238: F, t3282: F, t3294: F, t3295: F, t4721: F, t4726: F, t4731: F, t4735: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F, t4773: F) -> (F, F, F) {
    let t4776 = t136 * t4775;
    let t4778 = t1113 * t4733;
    let t4779 = t136 * t4778;
    let t4781 = -F::new(0.9494625e0) * t4749 + F::new(0.1898925e1) * t4757 + t3282 - F::new(0.99655555555555555557e-1) * t3238 - F::new(0.99655555555555555557e-1) * t4721 - F::new(0.19931111111111111111e0) * t4726 + F::new(0.59793333333333333334e0) * t4731 + F::new(0.29896666666666666667e0) * t4735 + F::new(0.15358125e0) * t4765 + F::new(0.3071625e0) * t4767 + t3294 - F::new(0.54771111111111111111e-1) * t3295 - F::new(0.54771111111111111111e-1) * t4770 - F::new(0.27385555555555555556e-1) * t4773 + F::new(0.16431333333333333333e0) * t4776 + F::new(0.82156666666666666667e-1) * t4779;
    (t4776, t4779, t4781)
}
