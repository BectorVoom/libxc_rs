//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 469/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk469<F: Float>(t1100: F, t1661: F, t1107: F, t1113: F, t1653: F, t136: F, t1105: F, t1112: F, t1655: F, t1118: F) -> (F, F, F, F, F, F) {
    let t1662 = t1100 * t1661;
    let t1665 = t1107 * t1661;
    let t1667 = t1113 * t1653;
    let t1668 = t136 * t1667;
    let t1670 = F::new(0.1898925e1) * t1662 - t1105 + F::new(0.29896666666666666667e0) * t1655 + F::new(0.3071625e0) * t1665 - t1112 + F::new(0.82156666666666666667e-1) * t1668;
    let t1671 = t1670 * t1118;
    (t1662, t1665, t1667, t1668, t1670, t1671)
}
