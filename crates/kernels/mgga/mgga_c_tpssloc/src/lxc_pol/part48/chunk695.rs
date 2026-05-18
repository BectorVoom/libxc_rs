//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 695/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk695<F: Float>(t1332: F, t1336: F, t22693: F, t22697: F, t22701: F, t22707: F, t22710: F, t22718: F, t22721: F, t22726: F, t22728: F, t22731: F, t22735: F, t3777: F, t6988: F, t6990: F) -> F {
    let t22739 = -t22693 - F::new(0.16449340668482264365e-1) * t22697 - F::new(0.82246703342411321825e-2) * t22701 + F::new(0.82246703342411321824e-2) * t22707 + F::new(2.0) * t1336 * t22710 - F::new(2.0) * t3777 * t6988 + t22718 + F::new(0.82246703342411321825e-2) * t22721 + t22726 - F::new(0.82246703342411321824e-2) * t22728 - t22731 + F::new(0.3289868133696452873e-1) * t22735 + F::new(2.0) * t1332 * t6990;
    t22739
}
