//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 283/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk283<F: Float>(t53: F, t60: F, t1375: F, t1378: F, t280: F, t814: F, t525: F, t990: F, t441: F, t50: F, t284: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1382 = piecewise3::<F>(t54, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1375 * t280 + F::new(4.0) / F::new(3.0) * t1378 * t814);
    let t1383 = t990 * t525;
    let t1386 = t441 * t50;
    let t1390 = piecewise3::<F>(t61, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1383 * t284 - F::new(4.0) / F::new(3.0) * t1386 * t814);
    let t1392 = t1382 / F::new(2.0) + t1390 / F::new(2.0);
    (t1383, t1386, t1392)
}
