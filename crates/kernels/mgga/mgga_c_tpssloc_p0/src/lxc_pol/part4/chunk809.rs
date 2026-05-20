//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 809/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk809<F: Float>(t3264: F, t5989: F, t1661: F, t3270: F, t3274: F, t4721: F, t5973: F, t5977: F, t5981: F, t1100: F, t3287: F, t1107: F) -> (F, F, F, F, F, F, F) {
    let t5991 = F::new(2.0) * t3264 * t5989;
    let t5992 = t1661 * t1661;
    let t5993 = t3270 * t5992;
    let t5999 = t3274 - F::new(2.0) / F::new(9.0) * t4721 - F::new(2.0) / F::new(9.0) * t5973 + F::new(2.0) / F::new(3.0) * t5977 + t5981 / F::new(3.0);
    let t6000 = t1100 * t5999;
    let t6006 = t3287 * t5992;
    let t6008 = t1107 * t5999;
    (t5991, t5992, t5993, t5999, t6000, t6006, t6008)
}
