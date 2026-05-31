//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1977/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1977<F: Float>(t87533: F, t87535: F, t87544: F, t87546: F, t87197: F, t87205: F, t87211: F, t81750: F, t84857: F, t84859: F, t87183: F, t87185: F, t87187: F, t87189: F, t87191: F, t87193: F, t87195: F, t87200: F, t87213: F, t87216: F, t87219: F) -> (F, F, F, F, F) {
    let t92560 = F::cast_from(0.15352717957250113407e0_f64) * t87533;
    let t92561 = F::cast_from(0.76763589786250567036e-1_f64) * t87535;
    let t92564 = F::cast_from(0.3289868133696452873e-1_f64) * t87544;
    let t92565 = F::cast_from(0.15352717957250113407e0_f64) * t87546;
    let t92578 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t87197;
    let t92580 = F::cast_from(0.56521858531796547194e-2_f64) * t87205;
    let t92582 = F::cast_from(0.13457585364713463618e-3_f64) * t87211;
    let t92586 = -t87183 / F::cast_from(384.0_f64) + t87185 / F::cast_from(96.0_f64) + t87187 / F::cast_from(96.0_f64) + t87189 / F::cast_from(96.0_f64) + t87191 / F::cast_from(96.0_f64) - t87193 / F::cast_from(768.0_f64) - t87195 / F::cast_from(384.0_f64) - t92578 + t87200 / F::cast_from(96.0_f64) - t92580 - t84857 + t84859 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t81750 + t92582 + F::cast_from(0.33643963411783659044e-4_f64) * t87213 + t87216 / F::cast_from(768.0_f64) + t87219 / F::cast_from(384.0_f64);
    (t92560, t92561, t92564, t92565, t92586)
}
