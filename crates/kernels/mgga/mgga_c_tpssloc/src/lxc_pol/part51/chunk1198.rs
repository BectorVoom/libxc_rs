//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1198/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1198<F: Float>(t120521: F, t114116: F, t114121: F, t32762: F, t6883: F, t1985: F, t214: F, t225: F, t26328: F, t567: F, t7722: F, t6907: F, t32761: F, t6897: F, t794: F, t114208: F) -> (F, F, F, F, F, F, F, F, F) {
    let t120522 = 0.82246703342411321825e-2 * t120521;
    let t120525 = 0.38381794893125283518e-1 * t114116;
    let t120526 = 0.82246703342411321825e-2 * t114121;
    let t120532 = t6883 * t32762;
    let t120533 = 0.38381794893125283518e-1 * t120532;
    let t120542 = 0.16449340668482264365e-1 * t1985 * t214 * t26328 * t225 * t567;
    let t120544 = t214 * t7722;
    let t120547 = 0.16449340668482264365e-1 * t1985 * t120544 * t6907;
    let t120550 = t6897 * t794 * t32761;
    let t120551 = 0.82246703342411321825e-2 * t120550;
    let t120552 = 0.76763589786250567036e-1 * t114208;
    (t120522, t120525, t120526, t120533, t120542, t120544, t120547, t120551, t120552)
}
