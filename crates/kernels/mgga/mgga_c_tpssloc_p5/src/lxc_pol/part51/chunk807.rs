//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 807/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk807<F: Float>(t1933: F, t7573: F, t1597: F, t343: F, t6734: F, t1615: F, t68: F, t360: F, t6744: F, t1611: F, t1941: F, t1607: F, t1618: F, t1622: F, t1935: F, t1937: F, t378: F, t6716: F, t6717: F, t6728: F, t6742: F, t6755: F, t6763: F, t6765: F) -> (F, F, F, F, F, F, F) {
    let t7574 = t1933 * t7573;
    let t7577 = t1597 * t343;
    let t7578 = t7577 * t6734;
    let t7581 = t1615 * t68;
    let t7582 = t7581 * t360;
    let t7583 = t6744 * t7582;
    let t7586 = t1611 * t1941;
    let t7593 = t6716 + t6717 * t1607 / F::cast_from(288.0_f64) + t6728 + F::cast_from(0.10093189023535097714e-3_f64) * t7574 * t1937 - F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t7578 + F::cast_from(0.10093189023535097714e-3_f64) * t6742 * t7583 + t7586 * t378 / F::cast_from(1536.0_f64) + t6755 * t1618 / F::cast_from(1536.0_f64) + t6763 + t6765 * t1622 / F::cast_from(2304.0_f64);
    (t7574, t7577, t7578, t7582, t7583, t7586, t7593)
}
