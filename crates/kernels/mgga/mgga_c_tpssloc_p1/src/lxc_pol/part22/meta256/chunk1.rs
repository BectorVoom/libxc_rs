//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1382/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1382<F: Float>(t1094: F, t3263: F, t11135: F, t11203: F, t11153: F, t461: F, t1176: F, t698: F) -> (F, F, F, F, F, F) {
    let t11424 = t1094 * t3263;
    let t11444 = F::cast_from(0.53272592592592592592e-1_f64) * t11135;
    let t11459 = F::cast_from(0.55403703703703703703e-1_f64) * t11135;
    let t11487 = F::new(20.0) / F::new(27.0) * t11203;
    let t11516 = t461 * t11153;
    let t11529 = t698 * t1176;
    (t11424, t11444, t11459, t11487, t11516, t11529)
}
