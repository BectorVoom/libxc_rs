//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 205/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk205<F: Float>(t53: F, t60: F, t1372: F, t433: F, t521: F, t983: F, t437: F, t50: F, t280: F, t814: F, t525: F, t990: F, t441: F, t284: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1373 = t1372 * t433;
    let t1374 = F::cast_from(0.5848223622634646207e0_f64) * t1373;
    let t1375 = t983 * t521;
    let t1378 = t437 * t50;
    let t1382 = piecewise3::<F>(t54, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1375 * t280 + F::new(4.0) / F::new(3.0) * t1378 * t814);
    let t1383 = t990 * t525;
    let t1386 = t441 * t50;
    let t1390 = piecewise3::<F>(t61, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1383 * t284 - F::new(4.0) / F::new(3.0) * t1386 * t814);
    (t1374, t1382, t1390)
}
