//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 866/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk866<F: Float>(t121634: F, t1484: F, t22986: F, t23270: F, t112676: F, t121296: F, t121305: F, t121308: F, t121454: F, t126226: F, t126229: F, t126233: F, t126240: F, t126246: F, t127778: F, t127786: F, t127790: F, t1528: F, t17052: F, t17092: F, t33399: F, t4147: F, t8563: F) -> (F,) {
    let t127794 = t22986 * t23270 * t121634 * t1484;
    let t127796 = -t126226 + t126229 + 0.38381794893125283518e-1 * t121296 + 0.82246703342411321824e-2 * t121305 + t126233 - 0.16449340668482264365e-1 * t121308 - 2.0 * t4147 * t33399 - 0.82246703342411321825e-2 * t127778 + t126240 - 2.0 * t17092 * t8563 - 2.0 * t121454 * t1528 - t17052 * t8563 - 0.16449340668482264365e-1 * t127786 - 0.49348022005446793095e-1 * t127790 + t126246 + 0.3289868133696452873e-1 * t127794 - t112676;
    (t127796,)
}
