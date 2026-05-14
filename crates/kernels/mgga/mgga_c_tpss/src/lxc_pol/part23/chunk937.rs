//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 937/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk937<F: Float>(t242: F, t3060: F, t3074: F, t1111: F, t461: F, t650: F, t1114: F, t3055: F, t3052: F, t3065: F, t8507: F, t3124: F, t3090: F, t774: F, t3069: F, t3067: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9537 = t242 * t3060 * t3074;
    let t9538 = t1111 * t9537;
    let t9540 = t650 * t461;
    let t9542 = t242 * t9540 * t1114;
    let t9543 = t1111 * t9542;
    let t9546 = t242 * t3060 * t3055;
    let t9547 = t3052 * t9546;
    let t9555 = t3065 * t8507;
    let t9556 = t3124 * t9555;
    let t9561 = t774 * t3090;
    let t9562 = t9561 * t3069;
    let t9563 = t3067 * t9562;
    (t9537, t9538, t9540, t9542, t9543, t9546, t9547, t9555, t9556, t9561, t9562, t9563)
}
