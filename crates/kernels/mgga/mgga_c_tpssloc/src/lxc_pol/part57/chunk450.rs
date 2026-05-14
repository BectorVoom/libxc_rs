//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 450/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk450<F: Float>(t136: F, t6017: F, t3282: F, t3294: F, t4721: F, t4770: F, t5973: F, t5977: F, t5981: F, t5993: F, t6000: F, t6006: F, t6008: F, t6012: F, t6015: F, t1118: F) -> (F, F) {
    let t6018 = t136 * t6017;
    let t6020 = -0.9494625e0 * t5993 + 0.1898925e1 * t6000 + t3282 - 0.19931111111111111111e0 * t4721 - 0.19931111111111111111e0 * t5973 + 0.59793333333333333334e0 * t5977 + 0.29896666666666666667e0 * t5981 + 0.15358125e0 * t6006 + 0.3071625e0 * t6008 + t3294 - 0.10954222222222222222e0 * t4770 - 0.27385555555555555556e-1 * t6012 + 0.16431333333333333333e0 * t6015 + 0.82156666666666666667e-1 * t6018;
    let t6021 = t6020 * t1118;
    (t6018, t6021)
}
