//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 849/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk849<F: Float>(t3: F, t8110: F, t1458: F, t577: F, t7423: F, t7768: F, t7771: F, t7773: F, t33: F, t68: F, t69: F, t79: F) -> (F, F, F, F, F) {
    let t8111 = t3 * t8110;
    let t8119 = F::cast_from(0.45e1_f64) * t8110 * t577 + F::cast_from(0.135e2_f64) * t7423 * t1458 + t7768 + t7771 + t7773;
    let t8301 = t33 * t33;
    let t8306 = F::cast_from(1.0_f64) / t69 / t68;
    let t8307 = t79 * t79;
    (t8111, t8119, t8301, t8306, t8307)
}
