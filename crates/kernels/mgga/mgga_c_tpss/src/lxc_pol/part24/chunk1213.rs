//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1213/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1213<F: Float>(t21132: F, t77: F, t4573: F, t578: F, t4580: F, t13298: F, t38: F, t18317: F, t18322: F, t4579: F, t5497: F, t72: F, t1679: F, t6086: F, t6090: F, t4622: F, t76: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21133 = t77 * t21132;
    let t21136 = t578 * t4573;
    let t21139 = t578 * t4580;
    let t21146 = t13298 * t38;
    let t21157 = 5.0 / 18.0 * t18317 * t4573 + 5.0 / 6.0 * t5497 * t4579 - t18322;
    let t21158 = t21157 * t72;
    let t21159 = t21158 * t1679;
    let t21162 = t6086 * t6090;
    let t21165 = t76 * t4622;
    (t21133, t21136, t21139, t21146, t21157, t21158, t21159, t21162, t21165)
}
