//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1883/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1883<F: Float>(t25060: F, t6547: F, t1880: F, t23237: F, t25216: F, t25192: F, t81651: F, t82074: F, t6552: F, t6555: F, t87782: F, t23270: F, t25038: F, t25191: F, t87036: F) -> (F, F, F, F, F) {
    let t87804 = t6547 * t25060;
    let t87822 = t1880 * t23237 * t25216;
    let t87835 = t81651 * t82074 * t25192;
    let t87861 = t6552 * t87782 * t6555;
    let t87866 = t25038 * t23270 * t25191 * t87036;
    (t87804, t87822, t87835, t87861, t87866)
}
