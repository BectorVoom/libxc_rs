//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 825/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk825<F: Float>(t20234: F, t9287: F, t3981: F, t5398: F, t20217: F, t43: F, t48: F, t481: F, t9300: F, t3990: F, t55: F, t1420: F, t1423: F, t39: F, t51: F, t5416: F, t5421: F, t5424: F, t56: F, t9311: F, sigma2: F) -> (F, F) {
    let t20235 = t9287 * t20234;
    let t20238 = t3981 * t5398;
    let t20241 = t43 * t20217;
    let t20245 = F::new(1.0) / t48 / t481;
    let t20246 = sigma2 * t20245;
    let t20255 = t9300 * t20234;
    let t20258 = t3990 * t5398;
    let t20261 = t55 * t20217;
    let t20264 = -F::new(5.0) / F::new(108.0) * t39 * t20235 + F::new(5.0) / F::new(6.0) * t39 * t20238 + F::new(5.0) / F::new(6.0) * t39 * t20241 - F::new(1232.0) / F::new(27.0) * t20246 * t56 - F::new(220.0) / F::new(9.0) * t5416 * t1423 - F::new(20.0) / F::new(9.0) * t1420 * t5421 + F::new(20.0) / F::new(3.0) * t1420 * t5424 + F::new(5.0) / F::new(108.0) * t51 * t20255 + F::new(5.0) / F::new(6.0) * t51 * t20258 - F::new(5.0) / F::new(6.0) * t51 * t20261 + t9311;
    (t20246, t20264)
}
