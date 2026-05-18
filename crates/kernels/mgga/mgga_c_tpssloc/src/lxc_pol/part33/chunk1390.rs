//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1390/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1390<F: Float>(t1339: F, t1824: F, t26288: F, t550: F, t6330: F, t1799: F, t22827: F, t6420: F, t1825: F, t6414: F, t6936: F, t107133: F, t107135: F, t107139: F, t107143: F, t107145: F, t107147: F, t107151: F, t107159: F, t107164: F, t80848: F, t80886: F, t91305: F, t91312: F, t91323: F, t91346: F, t97378: F, t97380: F) -> F {
    let t107169 = t26288 * t1339 * t6330 * t1824 * t550;
    let t107174 = t22827 * t1339 * t6420 * t1799;
    let t107178 = t6936 * t1339 * t1825 * t6414;
    let t107180 = -t107133 / F::new(384.0) - t107135 / F::new(128.0) - t80848 - F::new(0.67826230238155856634e-1) * t107139 - F::new(0.72670960969452703536e-2) * t107143 + t107145 / F::new(64.0) - F::new(5.0) / F::new(128.0) * t107147 + F::new(119.0) / F::new(2304.0) * t91305 - F::new(0.15812662803538319751e-2) * t91312 + t107151 / F::new(256.0) + F::new(7.0) / F::new(768.0) * t97378 - F::new(7.0) / F::new(384.0) * t97380 + F::new(0.3027956707060529314e-3) * t91323 + F::new(0.36335480484726351768e-2) * t107159 + F::new(0.36335480484726351768e-2) * t107164 - F::new(0.25434836339308446237e-1) * t107169 + F::new(0.50465945117675488567e-4) * t91346 - t80886 + F::new(0.36335480484726351768e-2) * t107174 - F::new(0.60559134141210586281e-3) * t107178;
    t107180
}
