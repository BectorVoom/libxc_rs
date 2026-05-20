//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1707/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1707<F: Float>(t1329: F, t22797: F, t2230: F, t6924: F, t213: F, t6928: F, t10: F, t2229: F, t60: F, t1995: F, t116: F, t117: F) -> (F, F, F, F, F, F, F, F) {
    let t22798 = t22797 * t1329;
    let t22803 = t2230 * t6924;
    let t22804 = t22803 * t213;
    let t22805 = t22804 * t6928;
    let t22811 = t2229 * t10;
    let t22813 = F::new(1.0) / t60 / t22811;
    let t22814 = t22813 * t1995;
    let t22815 = t117 * t116;
    (t22798, t22803, t22804, t22805, t22811, t22813, t22814, t22815)
}
