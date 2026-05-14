//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 819/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk819<F: Float>(t10294: F, t268: F, t271: F, t6546: F, t2394: F, t885: F) -> (F, F, F, F) {
    let t10542 = 0.36793333333333333333e0 * t10294;
    let t10544 = t268 * t6546 * t271;
    let t10545 = 0.93932222222222222223e0 * t10544;
    let t10556 = t2394 * t885;
    (t10542, t10544, t10545, t10556)
}
