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
    let t92597 = F::new(119.0) / F::new(3456.0) * t87243;
    let t92599 = F::new(7.0) / F::new(576.0) * t87247;
    let t92603 = F::new(7.0) / F::new(576.0) * t87255;
    let t92605 = -t87222 / F::new(192.0) - t87224 / F::new(96.0) - t87226 / F::new(192.0) - t92590 + F::new(5.0) / F::new(192.0) * t87235 - F::new(119.0) / F::new(432.0) * t81764 + F::new(7.0) / F::new(144.0) * t81770 + F::new(7.0) / F::new(288.0) * t81772 - F::cast_from(0.80745512188280781706e-3_f64) * t81785 + F::new(5.0) / F::new(96.0) * t87241 - t92597 - t87245 / F::new(768.0) + t92599 - t87249 / F::new(768.0) - t87251 / F::new(384.0) - t87253 / F::new(768.0) + t92603 - F::new(5.0) / F::new(32.0) * t87257;
    t92605
}
