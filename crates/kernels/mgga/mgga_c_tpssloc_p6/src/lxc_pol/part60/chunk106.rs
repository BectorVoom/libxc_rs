//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 106/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk106<F: Float>(t273: F, t276: F, t279: F, t285: F, t315: F, t293: F, t300: F, t302: F, t311: F, t194: F, t241: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t294 = F::cast_from(2.0_f64) <= zeta_threshold;
    let t297 = F::cast_from(0.0_f64) <= zeta_threshold;
    let t320 = F::cast_from(0.51785e1_f64) * t276 + F::cast_from(0.905775e0_f64) * t273 + F::cast_from(0.1100325e0_f64) * t279 + F::cast_from(0.1241775e0_f64) * t285;
    let t323 = F::cast_from(1.0_f64) + F::cast_from(0.29608749977793437516e2_f64) / t320;
    let t324 = F::ln(t323);
    let t325 = t315 * t324;
    let t328 = t300 * (-F::cast_from(0.310907e-1_f64) * t302 * t311 + t293 - F::cast_from(0.19751673498613801407e-1_f64) * t325);
    let t330 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t325;
    let t331 = piecewise3::<F>(t294, t194, t241);
    let t332 = piecewise3::<F>(t297, t194, F::cast_from(0.0_f64));
    let t334 = t331 / F::cast_from(2.0_f64) + t332 / F::cast_from(2.0_f64);
    let t335 = t334 * t334;
    (t320, t323, t324, t328, t330, t334, t335)
}
