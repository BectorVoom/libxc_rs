//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1376/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1376<F: Float>(t225: F, t23494: F, t1011: F, t3120: F, t3040: F, t6768: F, t23384: F, t23650: F, t1023: F, t1058: F, t1060: F, t10857: F, t10913: F, t11059: F, t11060: F, t1945: F, t23327: F, t23346: F, t23601: F, t23613: F, t23621: F, t23644: F, t23692: F, t23705: F, t25429: F, t25484: F, t25491: F, t25510: F, t25511: F, t25721: F, t3180: F, t3186: F, t3188: F, t3200: F, t3201: F, t4594: F, t6680: F, t6786: F, t82730: F) -> F {
    let t82750 = t23494 * t225;
    let t82754 = t3120 * t1011;
    let t82762 = t6768 * t3040;
    let t82789 = t23384 * t23650;
    let t82795 = -F::cast_from(0.82246703342411321826e-2_f64) * t23327 * t82750 * t6786 - F::cast_from(0.24674011002723396548e-1_f64) * t23601 * t25491 * t82754 * t1023 + t1058 * t1945 * t10857 * t1060 + F::new(6.0) * t3186 * t82762 * t3188 - F::new(3.0) * t3200 * t82762 * t3201 + F::new(3.0) * t3180 * t23705 - F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t25510 * t25511 * t10913 + F::cast_from(0.10966227112321509577e-1_f64) * t25429 * t25510 * t25721 * t10913 - F::cast_from(0.82246703342411321826e-2_f64) * t23327 * t23613 * t23692 + F::new(6.0) * t11059 * t82730 * t11060 + F::cast_from(0.49348022005446793095e-1_f64) * t23601 * t25484 * t82754 * t4594 - F::cast_from(0.82246703342411321826e-2_f64) * t82789 + F::cast_from(0.65797362673929057459e-1_f64) * t23346 * t23644 - F::cast_from(0.65797362673929057459e-1_f64) * t6680 * t23621;
    t82795
}
