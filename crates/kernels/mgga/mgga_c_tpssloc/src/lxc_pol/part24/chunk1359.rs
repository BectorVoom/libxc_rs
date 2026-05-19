//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1359/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1359<F: Float>(t3009: F, t984: F, t10309: F, t10321: F, t1065: F, t10913: F, t2250: F, t23313: F, t23327: F, t23329: F, t23330: F, t23336: F, t23346: F, t23353: F, t23365: F, t23366: F, t23593: F, t23728: F, t25423: F, t25429: F, t25430: F, t25797: F, t3010: F, t6687: F, t6689: F, t6690: F, t6691: F, t6692: F, t6699: F, t82342: F, t82343: F, t82357: F, t82380: F, t82382: F, t986: F) -> (F, F) {
    let t82385 = t3009 * t984;
    let t82389 = F::cast_from(0.10966227112321509577e-1_f64) * t25429 * t23329 * t25430 * t10913 - F::cast_from(0.82246703342411321826e-2_f64) * t23327 * t23336 * t23728 + F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t23329 * t82342 * t82343 - F::cast_from(0.82246703342411321826e-2_f64) * t23327 * t23329 * t23330 * t2250 * t1065 - F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t23329 * t25423 * t10913 - F::cast_from(0.82246703342411321826e-2_f64) * t23327 * t82357 * t6691 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t3010 * t6699 + F::cast_from(0.13159472534785811492e0_f64) * t23346 * t23366 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t23365 * t23313 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6689 * t6690 * t10321 - F::cast_from(0.21932454224643019154e-1_f64) * t6687 * t23593 * t6690 * t10309 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t986 * t23353 - F::cast_from(0.16449340668482264365e-1_f64) * t82380 + F::cast_from(0.80418998823691070229e-1_f64) * t82382 * t6692 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t82385 * t25797;
    (t82385, t82389)
}
