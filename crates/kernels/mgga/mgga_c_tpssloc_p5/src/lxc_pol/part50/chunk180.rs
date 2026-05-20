//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 180/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk180<F: Float>(t584: F, t9: F, t2: F, t16: F, t15: F, t3: F, t14: F) -> (F, F, F, F, F, F) {
    let t586 = F::new(0.174e1) * t9 * t584;
    let t587 = t9 * t2;
    let t588 = t587 * t16;
    let t589 = F::new(2.0) * t588;
    let t590 = t15 * t3;
    let t591 = F::new(1.0) / t590;
    let t592 = t14 * t591;
    (t586, t588, t589, t590, t591, t592)
}
