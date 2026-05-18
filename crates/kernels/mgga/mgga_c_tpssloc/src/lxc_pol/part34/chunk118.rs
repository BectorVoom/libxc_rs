//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 118/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk118<F: Float>(t273: F, t276: F, t279: F, t285: F, t315: F, t293: F, t300: F, t302: F, t311: F, t194: F, t241: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t294 = F::new(2.0) <= zeta_threshold;
    let t297 = F::new(0.0) <= zeta_threshold;
    let t320 = F::new(0.51785e1) * t276 + F::new(0.905775e0) * t273 + F::new(0.1100325e0) * t279 + F::new(0.1241775e0) * t285;
    let t323 = F::new(1.0) + F::new(0.29608749977793437516e2) / t320;
    let t324 = f64::ln(t323);
    let t325 = t315 * t324;
    let t328 = t300 * (-F::new(0.310907e-1) * t302 * t311 + t293 - F::new(0.19751673498613801407e-1) * t325);
    let t330 = F::new(0.19751673498613801407e-1) * t300 * t325;
    let t331 = piecewise3::<f64>(t294, t194, t241);
    let t332 = piecewise3::<f64>(t297, t194, F::new(0.0));
    let t334 = t331 / F::new(2.0) + t332 / F::new(2.0);
    let t335 = t334 * t334;
    (t320, t323, t324, t328, t330, t334, t335)
}
