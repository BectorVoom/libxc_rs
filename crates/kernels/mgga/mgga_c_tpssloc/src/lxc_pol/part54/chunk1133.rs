//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1133/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1133<F: Float>(t10109: F, t7106: F, t10143: F, t7844: F, t225: F, t27137: F, t27059: F, t2091: F, t40590: F, t27070: F, t27052: F, t2085: F, t5286: F, t1824: F, t7191: F, t12020: F, t7213: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t92981 = t10109 * t7106;
    let t93000 = t7844 * t10143;
    let t93313 = t27137 * t225;
    let t93316 = t27059 * t225;
    let t93319 = t40590 * t2091;
    let t93338 = t27070 * t225;
    let t93341 = t27052 * t225;
    let t93501 = t2085 * t5286;
    let t93505 = t7191 * t1824;
    let t93818 = t12020 * t7213;
    (t92981, t93000, t93313, t93316, t93319, t93338, t93341, t93501, t93505, t93818)
}
