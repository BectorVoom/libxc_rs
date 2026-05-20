//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1225/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1225<F: Float>(t1230: F, t248: F, t4733: F, t3440: F, t4724: F, t1193: F, t1706: F, t135: F, t1725: F, t1174: F, t1196: F, t3966: F) -> (F, F, F, F, F) {
    let t5030 = t248 * t1230 * t4733;
    let t5033 = t3440 * t4724;
    let t5036 = t1706 * t1193;
    let t5040 = t135 * t1725;
    let t5041 = t1174 * t5040;
    let t5045 = t1196 * t3966;
    (t5030, t5033, t5036, t5041, t5045)
}
