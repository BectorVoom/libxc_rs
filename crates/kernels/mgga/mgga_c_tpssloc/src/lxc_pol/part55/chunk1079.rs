//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1079/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1079<F: Float>(t11606: F, t32537: F, t24574: F, t8872: F, t2144: F, t7299: F, t7302: F, t1186: F, t8867: F, t1238: F, t2155: F, t24589: F, t24880: F, t24893: F, t32511: F, t32516: F, t32520: F, t32524: F, t32530: F, t3487: F, t7283: F, t7351: F, t7392: F, t8888: F) -> (F, F, F, F, F, F) {
    let t32538 = t11606 * t32537;
    let t32542 = F::new(0.54831135561607547883e-2) * t24574 * t8872;
    let t32543 = t7299 * t2144;
    let t32544 = t32543 * t7302;
    let t32547 = t1186 * t8867;
    let t32550 = -F::new(2.0) * t24893 * t2155 + F::new(0.3289868133696452873e-1) * t7283 * t32511 + F::new(0.16449340668482264365e-1) * t7283 * t32516 - F::new(0.54831135561607547883e-2) * t7283 * t32520 + F::new(0.54831135561607547883e-2) * t24589 * t32524 - F::new(2.0) * t24880 * t2155 - F::new(0.16449340668482264365e-1) * t7283 * t32530 - F::new(2.0) * t7351 * t7392 + F::new(2.0) * t3487 * t8888 - F::new(6.0) * t1238 * t32538 - t32542 - F::new(0.16449340668482264365e-1) * t7283 * t32544 - F::new(0.16449340668482264365e-1) * t7283 * t32547;
    (t32538, t32542, t32543, t32544, t32547, t32550)
}
