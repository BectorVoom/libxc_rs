//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2114/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2114<F: Float>(t23053: F, t4236: F, t13173: F, t6614: F, t23041: F, t13186: F, t6621: F, t81770: F, t81772: F, t81785: F, t87222: F, t87224: F, t87226: F, t87234: F, t87235: F, t87237: F, t87241: F, t87243: F, t87245: F, t87248: F, t87249: F) -> F {
    let t87251 = t23053 * t4236;
    let t87253 = t6614 * t13173;
    let t87255 = t23041 * t4236;
    let t87256 = F::new(7.0) / F::new(1152.0) * t87255;
    let t87257 = t6621 * t13186;
    let t87259 = -t87222 / F::new(384.0) - t87224 / F::new(192.0) - t87226 / F::new(384.0) - t87234 + F::new(5.0) / F::new(384.0) * t87235 - t87237 + F::new(7.0) / F::new(288.0) * t81770 + F::new(7.0) / F::new(576.0) * t81772 - F::cast_from(0.40372756094140390854e-3_f64) * t81785 + F::new(5.0) / F::new(192.0) * t87241 - F::new(119.0) / F::new(6912.0) * t87243 - t87245 / F::new(1536.0) + t87248 - t87249 / F::new(1536.0) - t87251 / F::new(768.0) - t87253 / F::new(1536.0) + t87256 - F::new(5.0) / F::new(64.0) * t87257;
    t87259
}
