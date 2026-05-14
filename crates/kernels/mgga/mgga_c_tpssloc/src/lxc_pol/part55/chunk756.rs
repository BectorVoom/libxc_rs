//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 756/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk756<F: Float>(t1997: F, t8462: F, t240: F, t553: F, t544: F, t1342: F, t248: F) -> (F, F, F, F) {
    let t8463 = t1997 * t8462;
    let t8465 = t553 * t240;
    let t8466 = t544 * t8465;
    let t8467 = t1342 * t248;
    (t8463, t8465, t8466, t8467)
}
