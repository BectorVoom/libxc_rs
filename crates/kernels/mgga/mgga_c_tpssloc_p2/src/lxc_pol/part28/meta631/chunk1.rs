//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1978/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1978<F: Float>(t87233: F, t87243: F, t87247: F, t87255: F, t81764: F, t81770: F, t81772: F, t81785: F, t87222: F, t87224: F, t87226: F, t87235: F, t87241: F, t87245: F, t87249: F, t87251: F, t87253: F, t87257: F) -> F {
    let t92590 = F::cast_from(0.26915170729426927236e-3_f64) * t87233;
    let t92597 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t87243;
    let t92599 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t87247;
    let t92603 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t87255;
    let t92605 = -t87222 / F::cast_from(192.0_f64) - t87224 / F::cast_from(96.0_f64) - t87226 / F::cast_from(192.0_f64) - t92590 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t87235 - F::cast_from(119.0_f64) / F::cast_from(432.0_f64) * t81764 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t81770 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t81772 - F::cast_from(0.80745512188280781706e-3_f64) * t81785 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t87241 - t92597 - t87245 / F::cast_from(768.0_f64) + t92599 - t87249 / F::cast_from(768.0_f64) - t87251 / F::cast_from(384.0_f64) - t87253 / F::cast_from(768.0_f64) + t92603 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t87257;
    t92605
}
