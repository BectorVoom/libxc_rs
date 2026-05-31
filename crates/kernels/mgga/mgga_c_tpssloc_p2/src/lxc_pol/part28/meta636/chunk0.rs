//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2020/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2020<F: Float>(t91214: F, t80761: F, t80767: F, t80769: F, t80776: F, t91183: F, t91185: F, t91187: F, t91189: F, t91192: F, t91196: F, t91200: F, t91204: F, t91206: F, t91210: F, t91212: F, t91216: F, t91218: F) -> F {
    let t93674 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t91214;
    let t93681 = -F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t91183 - t91185 / F::cast_from(768.0_f64) - t91187 / F::cast_from(384.0_f64) - t91189 / F::cast_from(768.0_f64) - t91192 / F::cast_from(96.0_f64) - t91196 / F::cast_from(2.0_f64) - F::cast_from(0.13565246047631171326e0_f64) * t91200 + F::cast_from(0.48447307312968469024e-2_f64) * t91204 - F::cast_from(0.63250651214153279003e-2_f64) * t91206 - F::cast_from(0.33913115119077928316e-1_f64) * t91210 - t91212 / F::cast_from(96.0_f64) - t93674 - t91216 / F::cast_from(768.0_f64) - t91218 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t80761 - F::cast_from(0.27130492095262342653e0_f64) * t80767 + F::cast_from(0.16956557559538964158e-1_f64) * t80769 - F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t80776;
    t93681
}
