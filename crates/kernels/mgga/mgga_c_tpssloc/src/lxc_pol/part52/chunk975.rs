//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 975/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk975<F: Float>(t4017: F, t71: F, t1863: F, t12568: F, t33: F, t1409: F, t22502: F, t22505: F, t22510: F, t3961: F, t3966: F, t6500: F, t67: F, t1864: F, t6509: F, t7441: F) -> (F, F, F, F, F, F) {
    let t26024 = t71 * t4017;
    let t26025 = t1863 * t26024;
    let t26028 = t12568 * t33;
    let t26043 = -20.0 / 9.0 * t22502 * t1409 + 5.0 / 18.0 * t22505 * t3961 + 5.0 / 6.0 * t6500 * t3966 - t22510;
    let t26044 = t26043 * t67;
    let t26045 = t26044 * t1864;
    let t26048 = t7441 * t6509;
    (t26024, t26025, t26028, t26043, t26045, t26048)
}
