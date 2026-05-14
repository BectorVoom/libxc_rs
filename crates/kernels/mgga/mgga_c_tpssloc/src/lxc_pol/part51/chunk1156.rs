//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1156/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1156<F: Float>(t114932: F, t31420: F, t6547: F, t23171: F, t23228: F, t8547: F, t31370: F, t23204: F, t31419: F, t6562: F, t2752: F, t31429: F, t193: F, t201: F, t8565: F, t10143: F) -> (F, F, F, F, F, F, F, F) {
    let t114933 = 0.82246703342411321824e-2 * t114932;
    let t114939 = t6547 * t31420;
    let t114943 = t23171 * t23228 * t8547;
    let t114944 = 0.82246703342411321824e-2 * t114943;
    let t114945 = t6547 * t31370;
    let t114965 = t6562 * t23204 * t31419;
    let t114992 = t31429 * t2752;
    let t115009 = t193 * t201 * t8565;
    let t115027 = t8565 * t10143;
    (t114933, t114939, t114944, t114945, t114965, t114992, t115009, t115027)
}
