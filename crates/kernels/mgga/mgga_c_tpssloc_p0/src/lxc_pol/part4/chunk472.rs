//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 472/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk472<F: Float>(t1687: F, t449: F, t1150: F, t1153: F, t1655: F, t1662: F, t1665: F, t1668: F) -> (F, F) {
    let t1688 = t1687 * t449;
    let t1694 = F::new(0.258925e1) * t1662 - t1150 + F::new(0.301925e0) * t1655 + F::new(0.16504875e0) * t1665 - t1153 + F::new(0.82785e-1) * t1668;
    (t1688, t1694)
}
