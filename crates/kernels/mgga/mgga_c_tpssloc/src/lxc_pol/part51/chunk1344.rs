//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1344/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1344<F: Float>(t114105: F, t1985: F, t1998: F, t214: F, t26328: F, t32749: F, t6883: F, t32748: F, t6897: F, t794: F, t114116: F, t114121: F) -> (F, F, F, F, F, F) {
    let t120507 = F::new(0.38381794893125283518e-1) * t114105;
    let t120513 = F::new(0.16449340668482264365e-1) * t1985 * t214 * t1998 * t26328;
    let t120514 = t6883 * t32749;
    let t120515 = F::new(0.38381794893125283518e-1) * t120514;
    let t120521 = t6897 * t794 * t32748;
    let t120522 = F::new(0.82246703342411321825e-2) * t120521;
    let t120525 = F::new(0.38381794893125283518e-1) * t114116;
    let t120526 = F::new(0.82246703342411321825e-2) * t114121;
    (t120507, t120513, t120515, t120522, t120525, t120526)
}
