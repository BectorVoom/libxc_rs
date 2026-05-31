//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1280/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1280<F: Float>(t10213: F, t241: F, t136: F, t41667: F, t41671: F, t908: F, t10319: F, t699: F, t10313: F, t2826: F, t41649: F, t41654: F) -> (F, F, F, F, F, F) {
    let t41880 = t241 * t10213;
    let t41882 = t136 * t41880 * t41667;
    let t41885 = t136 * t908 * t41671;
    let t41887 = t699 * t10319;
    let t41889 = t699 * t10313;
    let t41892 = t136 * t2826 * t41649;
    let t41904 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t41654;
    (t41882, t41885, t41887, t41889, t41892, t41904)
}
