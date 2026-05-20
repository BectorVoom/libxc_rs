//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2330/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2330<F: Float>(t4233: F, t9975: F, t13397: F, t1510: F, t16679: F, t16815: F, t16816: F, t16828: F, t16830: F, t16935: F, t17027: F, t17028: F, t20806: F, t2617: F, t4166: F, t4234: F, t4281: F, t59347: F, t67358: F, t67441: F, t67568: F, t812: F, t860: F, t861: F) -> (F, F) {
    let t67578 = t9975 * t4233;
    let t67582 = -F::new(18.0) * t13397 * t16815 * t67578 - F::new(18.0) * t13397 * t16816 * t67358 - F::new(3.0) * t1510 * t59347 * t812 + F::new(18.0) * t16815 * t16935 * t4281 - F::new(3.0) * t17027 * t4234 * t812 - t67568 * t812 * t860 - F::new(6.0) * t16679 * t4166 - F::new(3.0) * t16828 * t16830 - F::new(3.0) * t17028 * t4166 - F::new(3.0) * t20806 * t2617 - t67441 * t861;
    (t67578, t67582)
}
