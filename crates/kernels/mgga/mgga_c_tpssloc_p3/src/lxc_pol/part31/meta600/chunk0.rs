//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1845/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1845<F: Float>(t26653: F, t814: F, t87520: F, t87522: F, t87533: F, t87535: F, t87544: F, t87546: F, t87197: F, t87205: F, t87211: F, t87233: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t92546 = t814 * t26653;
    let t92551 = F::cast_from(0.3289868133696452873e-1_f64) * t87520;
    let t92556 = F::cast_from(0.15352717957250113407e0_f64) * t87522;
    let t92560 = F::cast_from(0.15352717957250113407e0_f64) * t87533;
    let t92561 = F::cast_from(0.76763589786250567036e-1_f64) * t87535;
    let t92564 = F::cast_from(0.3289868133696452873e-1_f64) * t87544;
    let t92565 = F::cast_from(0.15352717957250113407e0_f64) * t87546;
    let t92578 = F::new(7.0) / F::new(144.0) * t87197;
    let t92580 = F::cast_from(0.56521858531796547194e-2_f64) * t87205;
    let t92582 = F::cast_from(0.13457585364713463618e-3_f64) * t87211;
    let t92590 = F::cast_from(0.26915170729426927236e-3_f64) * t87233;
    (t92546, t92551, t92556, t92560, t92561, t92564, t92565, t92578, t92580, t92582, t92590)
}
