//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 839/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk839<F: Float>(t30: F, t259: F, t379: F, t1897: F, t645: F, t5664: F, t1867: F, t45: F, t5598: F, t581: F, t1095: F, t1872: F, t1871: F, t762: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t5991 = t1897 * t645;
    let t5994 = piecewise3(t380, 0.0, t5664);
    let t5999 = piecewise3(t120, t5598, t1867 * t581 / 2.0 + t5994 * t45 / 2.0);
    let t6001 = t1872 * t1095 / 288.0;
    let t6002 = t1871 * t762;
    (t5991, t5994, t5999, t6001, t6002)
}
