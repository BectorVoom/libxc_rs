//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1312/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1312<F: Float>(t1484: F, t1902: F, t22986: F, t6646: F, t829: F, t112968: F, t25261: F, t2647: F, t112974: F, t32826: F, t6562: F, t794: F) -> (F, F, F, F, F, F) {
    let t118690 = t1902 * t1484;
    let t118694 = F::new(0.3289868133696452873e-1) * t22986 * t6646 * t118690 * t829;
    let t118695 = F::new(0.76763589786250567036e-1) * t112968;
    let t118699 = F::new(0.3289868133696452873e-1) * t22986 * t6646 * t25261 * t2647;
    let t118700 = F::new(0.38381794893125283518e-1) * t112974;
    let t118709 = t6562 * t794 * t32826;
    (t118690, t118694, t118695, t118699, t118700, t118709)
}
