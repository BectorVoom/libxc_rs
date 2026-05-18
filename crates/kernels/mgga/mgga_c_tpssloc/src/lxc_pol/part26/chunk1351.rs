//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1351/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1351<F: Float>(t3590: F, t7284: F, t11545: F, t461: F, t491: F, t24574: F, t24630: F, t24605: F, t85639: F, t24888: F, t10913: F, t11148: F, t11172: F, t11606: F, t11608: F, t1238: F, t24563: F, t24567: F, t24589: F, t24590: F, t24601: F, t24633: F, t24883: F, t24887: F, t24897: F, t27444: F, t3593: F, t3599: F, t7283: F, t7285: F, t7286: F, t7287: F, t7351: F, t7391: F) -> (F, F) {
    let t85750 = t7284 * t3590;
    let t85754 = t11545 * t461;
    let t85755 = t85754 * t491;
    let t85766 = t24574 * t24630;
    let t85787 = t85639 * t24605;
    let t85789 = t24574 * t24888;
    let t85791 = -F::new(0.82246703342411321826e-2) * t7283 * t85750 * t7287 - F::new(0.8529287754027840782e-2) * t7283 * t85755 * t7286 * t11148 - F::new(18.0) * t1238 * t11606 * t7391 * t3599 - F::new(6.0) * t7351 * t11608 - F::new(0.16449340668482264365e-1) * t85766 - F::new(0.24674011002723396548e-1) * t7283 * t24567 * t24563 - F::new(18.0) * t3593 * t24897 - F::new(0.27415567780803773942e-2) * t7283 * t7285 * t7286 * t11172 - F::new(0.16449340668482264365e-1) * t7283 * t24633 * t24887 + F::new(0.82246703342411321826e-2) * t24589 * t24590 * t24883 - F::new(0.16449340668482264365e-1) * t24589 * t24601 * t27444 * t10913 + F::new(0.54831135561607547883e-2) * t85787 - F::new(0.54831135561607547883e-2) * t85789;
    (t85754, t85791)
}
