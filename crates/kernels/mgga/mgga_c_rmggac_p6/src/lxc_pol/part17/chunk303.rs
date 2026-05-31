//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 303/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk303<F: Float>(t53: F, t60: F, t1818: F, t196: F, t1794: F, t1797: F, t437: F, t983: F, t1802: F, t1805: F, t441: F, t990: F, zeta_threshold: F) -> (F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1819 = t196 * t1818;
    let t1827 = piecewise3::<F>(t54, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t983 * t1794 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t437 * t1797);
    let t1833 = piecewise3::<F>(t61, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t990 * t1802 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t441 * t1805);
    let t1835 = t1827 / F::cast_from(2.0_f64) + t1833 / F::cast_from(2.0_f64);
    (t1819, t1835)
}
