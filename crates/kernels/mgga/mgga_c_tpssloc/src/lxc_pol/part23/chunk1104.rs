//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1104/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1104<F: Float>(t19815: F, t3802: F, t12365: F, t6417: F, t40018: F, t6371: F, t12189: F, t6375: F, t40281: F, t6396: F, t12345: F, t6427: F, t6431: F, t3865: F, t3789: F, t40159: F, t6390: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t56878 = t19815 * t3802;
    let t56927 = t12365 * t6417;
    let t56946 = t40018 * t6371;
    let t56953 = t12189 * t6375;
    let t56993 = t40281 * t6396;
    let t57011 = t12345 * t6427;
    let t57019 = t12345 * t6431;
    let t57021 = t19815 * t3865;
    let t57033 = t19815 * t3789;
    let t57041 = t40159 * t6390;
    (t56878, t56927, t56946, t56953, t56993, t57011, t57019, t57021, t57033, t57041)
}
