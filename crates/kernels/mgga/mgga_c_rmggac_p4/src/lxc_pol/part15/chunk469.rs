//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 469/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk469<F: Float>(t53: F, t1794: F, t3985: F, t1797: F, t912: F, t3878: F, t814: F, t1395: F, t280: F, t57: F, t815: F, t1802: F, t3998: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t5850 = t3985 * t1794;
    let t5855 = t912 * t1797;
    let t5860 = -F::cast_from(2.0_f64) * t814 - F::cast_from(6.0_f64) * t3878;
    let t5864 = piecewise3::<F>(t54, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5850 * t280 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1395 * t815 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5855 * t280 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t57 * t5860);
    let t5865 = t3998 * t1802;
    (t5860, t5864, t5865)
}
