//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1056/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1056<F: Float>(t6581: F, t805: F, t1891: F, t2230: F, t213: F, t1895: F, t202: F, t243: F) -> (F, F, F, F) {
    let t6582 = t6581 * t805;
    let t6584 = t2230 * t1891;
    let t6585 = t6584 * t213;
    let t6586 = t6585 * t1895;
    let t6589 = F::new(1.0) / t243 / t202;
    (t6582, t6584, t6586, t6589)
}
