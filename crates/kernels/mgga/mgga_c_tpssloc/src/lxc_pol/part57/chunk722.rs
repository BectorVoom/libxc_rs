//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 722/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk722<F: Float>(t23109: F, t25132: F, t1496: F, t23069: F, t1512: F, t23041: F, t4166: F, t6613: F, t253: F, t254: F, t10109: F, t1911: F) -> (F, F, F, F, F, F) {
    let t25133 = t23109 * t25132;
    let t25140 = t23069 * t1496;
    let t25144 = t23041 * t1512;
    let t25146 = t4166 * t6613;
    let t25168 = t253 * t254;
    let t25169 = t10109 * t1911;
    (t25133, t25140, t25144, t25146, t25168, t25169)
}
