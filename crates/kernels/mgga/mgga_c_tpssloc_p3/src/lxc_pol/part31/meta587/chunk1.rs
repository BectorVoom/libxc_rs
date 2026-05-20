//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1830/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1830<F: Float>(t26289: F, t6604: F, t80887: F, t16060: F, t6951: F, t1878: F, t80730: F, t80893: F, t6925: F, t6976: F, t26271: F, t80779: F) -> (F, F, F, F, F, F) {
    let t91179 = t80887 * t6604 * t26289;
    let t91191 = t16060 * t6951;
    let t91194 = t1878 * t80730;
    let t91198 = t80893 * t6604;
    let t91202 = t6925 * t6976;
    let t91206 = t80779 * t26271;
    (t91179, t91191, t91194, t91198, t91202, t91206)
}
