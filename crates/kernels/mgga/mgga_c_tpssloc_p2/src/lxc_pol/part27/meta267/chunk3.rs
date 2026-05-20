//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1283/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1283<F: Float>(t1611: F, t1941: F, t1607: F, t1618: F, t1622: F, t1935: F, t1937: F, t378: F, t6716: F, t6717: F, t6728: F, t6742: F, t6755: F, t6763: F, t6765: F, t7574: F, t7578: F, t7583: F) -> (F, F) {
    let t7586 = t1611 * t1941;
    let t7593 = t6716 + t6717 * t1607 / F::new(288.0) + t6728 + F::cast_from(0.10093189023535097714e-3_f64) * t7574 * t1937 - F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t7578 + F::cast_from(0.10093189023535097714e-3_f64) * t6742 * t7583 + t7586 * t378 / F::new(1536.0) + t6755 * t1618 / F::new(1536.0) + t6763 + t6765 * t1622 / F::new(2304.0);
    (t7586, t7593)
}
