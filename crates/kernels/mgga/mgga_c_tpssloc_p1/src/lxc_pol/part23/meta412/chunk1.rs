//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1230/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1230<F: Float>(t12923: F, t4194: F, t5398: F, t20800: F, t262: F, t10143: F, t20778: F, t13115: F, t16586: F, t21038: F, t225: F, t21061: F) -> (F, F, F, F, F, F) {
    let t67230 = t4194 * t12923 * t5398;
    let t67235 = t262 * t20800;
    let t67239 = t20778 * t10143;
    let t67243 = t13115 * t16586;
    let t67305 = t21038 * t225;
    let t67339 = t21061 * t225;
    (t67230, t67235, t67239, t67243, t67305, t67339)
}
