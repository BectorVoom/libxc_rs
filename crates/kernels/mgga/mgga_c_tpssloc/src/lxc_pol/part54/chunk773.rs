//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 773/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk773<F: Float>(t1245: F, t7376: F, t7375: F, t1235: F, t2147: F, t462: F, t1215: F, t2144: F, t1246: F, t493: F, t7348: F, t1201: F, t1244: F, t2121: F, t2152: F, t470: F, t7283: F, t7361: F, t7365: F, t7368: F, t7373: F) -> (F, F, F, F, F, F, F) {
    let t7377 = t1245 * t7376;
    let t7378 = t7375 * t7377;
    let t7381 = t2147 * t1235;
    let t7382 = t462 * t7381;
    let t7386 = t2144 * t1215;
    let t7387 = t7386 * t1246;
    let t7389 = t493 * t7348;
    let t7391 = t7361 - F::new(0.27415567780803773942e-2) * t7283 * t7365 - F::new(0.82246703342411321825e-2) * t7283 * t7368 + F::new(0.82246703342411321825e-2) * t7373 * t7378 + F::new(0.82246703342411321825e-2) * t2121 * t7382 + t1201 * t2152 + t1244 * t7387 + t470 * t7389;
    (t7377, t7378, t7381, t7382, t7387, t7389, t7391)
}
