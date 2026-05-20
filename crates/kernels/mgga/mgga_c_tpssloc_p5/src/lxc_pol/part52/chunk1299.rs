//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1299/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1299<F: Float>(t118509: F, t118634: F, t118668: F, t118793: F, t118832: F, t118878: F, t118914: F, t118945: F, t870: F, t7540: F, t868: F, t25373: F) -> (F, F, F, F) {
    let t118948 = t118509 + t118634 + t118668 + t118793 + t118832 + t118878 + t118914 + t118945;
    let t118949 = t118948 * t870;
    let t118953 = t7540 * t868;
    let t118954 = t25373 * t118953;
    (t118948, t118949, t118953, t118954)
}
