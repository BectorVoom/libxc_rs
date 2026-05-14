//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 596/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk596<F: Float>(t109: F, t3: F, t8110: F, t1458: F, t577: F, t7423: F, t7768: F, t7771: F, t7773: F, t33: F, t68: F, t69: F, t79: F) -> (F, F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t8111 = t3 * t8110;
    let t8119 = 0.45e1 * t8110 * t577 + 0.135e2 * t7423 * t1458 + t7768 + t7771 + t7773;
    let t8301 = t33 * t33;
    let t8306 = 1.0 / t69 / t68;
    let t8307 = t79 * t79;
    let t8308 = t8306 * t8307;
    let t8326 = piecewise3(t110, 0.0, 0.0);
    (t8111, t8119, t8301, t8306, t8307, t8308, t8326)
}
