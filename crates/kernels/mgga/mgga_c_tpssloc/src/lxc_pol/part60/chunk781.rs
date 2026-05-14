//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 781/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk781<F: Float>(t131: F, t7245: F, t2240: F, t31: F, t63: F, t79: F, t625: F, t8307: F, t8513: F, t8663: F, t111: F, t8828: F) -> (F, F, F, F, F, F, F, F) {
    let t31863 = t7245 * t131;
    let t31864 = t2240 * t31863;
    let t32331 = t63 * t31;
    let t32338 = t79 * t63;
    let t32343 = t8307 * t625;
    let t32344 = t8513 * t32343;
    let t32346 = 5.0 / 27.0 * t8663 * t32344;
    let t32350 = t8828 * t111;
    (t31863, t31864, t32331, t32338, t32343, t32344, t32346, t32350)
}
