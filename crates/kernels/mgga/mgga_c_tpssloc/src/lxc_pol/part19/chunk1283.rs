//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1283/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1283<F: Float>(t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t11778: F, t154: F, t123: F, t43764: F) -> (F, F) {
    let t43808 = -16.0 / 27.0 * t43748 - 40.0 / 81.0 * t43750 + 8.0 / 9.0 * t43780 + 16.0 / 9.0 * t43782 + 16.0 / 9.0 * t43784 - 8.0 / 3.0 * t43786 - 4.0 / 9.0 * t43788 + 40.0 / 9.0 * t43794 - 8.0 * t43798 + 8.0 * t43802 + t43806 / 3.0;
    let t43809 = t154 * t11778;
    let t43811 = t123 * t43809 * t43764;
    (t43808, t43811)
}
