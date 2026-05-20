//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2272/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2272<F: Float>(t5837: F, t984: F, t23384: F, t28691: F, t28705: F, t82431: F, t14545: F, t1635: F, t18070: F, t1956: F, t23327: F, t23336: F, t23372: F, t25420: F, t25429: F, t25750: F, t25797: F, t28491: F, t4557: F, t5944: F, t61646: F, t6687: F, t6704: F, t7565: F, t7600: F, t82481: F, t88162: F, t88167: F, t88194: F, t88744: F, t89598: F) -> (F, F) {
    let t99180 = t5837 * t984;
    let t99184 = t23384 * t28691;
    let t99190 = t82431 * t28705;
    let t99202 = -F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t89598 * t7565 + F::new(4.0) * t4557 * t25420 + F::new(4.0) * t14545 * t7600 - t88167 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t99180 * t25797 - F::cast_from(0.27415567780803773942e-2_f64) * t99184 - t23372 * t5944 - F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t23336 * t28491 - F::cast_from(0.18277045187202515961e-2_f64) * t99190 - F::new(2.0) * t88744 * t1635 - F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t6704 * t82481 * t18070 - t61646 * t1956 + t88194 + F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t88162 * t25750;
    (t99180, t99202)
}
