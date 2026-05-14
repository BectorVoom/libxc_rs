//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1199/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1199<F: Float>(t225: F, t29685: F, t103345: F, t2122: F, t24574: F, t29674: F, t29750: F, t85853: F, t1011: F, t6218: F, t29624: F, t29614: F, t24826: F, t29782: F, t29736: F, t86094: F) -> (F, F, F, F, F, F, F, F, F) {
    let t103464 = t29685 * t225;
    let t103490 = t2122 * t103345;
    let t103494 = t24574 * t29674;
    let t103507 = t85853 * t29750;
    let t103515 = t6218 * t1011;
    let t103520 = t29624 * t225;
    let t103528 = t29614 * t225;
    let t103546 = t24826 * t29782;
    let t103573 = t86094 * t29736;
    (t103464, t103490, t103494, t103507, t103515, t103520, t103528, t103546, t103573)
}
