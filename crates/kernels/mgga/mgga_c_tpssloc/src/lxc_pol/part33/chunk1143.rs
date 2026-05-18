//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1143/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1143<F: Float>(t552: F, t6604: F, t1338: F, t7722: F, t7696: F, t794: F, t6897: F, t225: F, t7704: F, t112: F, t7758: F, t25049: F) -> (F, F, F, F, F, F, F) {
    let t26446 = t6604 * t552;
    let t26458 = t1338 * t7722;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26477 = t7704 * t225;
    let t26523 = t7758 * t112;
    let t26591 = F::new(0.38381794893125283518e-1) * t25049;
    (t26446, t26458, t26474, t26475, t26477, t26523, t26591)
}
