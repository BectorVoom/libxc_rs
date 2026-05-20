//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1834/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1834<F: Float>(t1361: F, t22690: F, t22792: F, t5187: F, t1307: F, t7708: F, t80840: F, t90787: F, t26245: F, t80783: F, t22897: F, t6925: F) -> (F, F, F, F) {
    let t91327 = t22792 * t22690 * t1361 * t5187;
    let t91344 = t80840 * t90787 * t7708 * t1307;
    let t91346 = t80783 * t26245;
    let t91351 = t6925 * t22897;
    (t91327, t91344, t91346, t91351)
}
