//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 394/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk394<F: Float>(t15: F, t60: F, t59: F, t207: F, t215: F, t782: F, t786: F, t789: F, t591: F, t795: F, t154: F, t244: F) -> (F, F, F, F, F, F, F, F) {
    let t2558 = F::new(1.0) / t60 / t15;
    let t2559 = t59 * t2558;
    let t2562 = F::new(0.64814814814814814813e-2) * t2559 * t207 * t215;
    let t2563 = t782 * t786;
    let t2564 = t2563 * t789;
    let t2566 = t59 * t591;
    let t2569 = F::new(0.26388888888888888888e-2) * t2566 * t207 * t795;
    let t2570 = t154 * t244;
    (t2558, t2559, t2562, t2563, t2564, t2566, t2569, t2570)
}
