//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 587/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk587<F: Float>(t225: F, t5600: F, t2671: F, t5527: F, t5544: F, t824: F, t1504: F, t1506: F, t228: F, t230: F) -> (F, F, F, F) {
    let t5601 = t5600 * t225;
    let t5605 = t2671 * t5527;
    let t5608 = t824 * t5544;
    let t5611 = F::new(6.0) * t1504 * t1506 - F::new(12.0) * t228 * t5605 + F::new(3.0) * t228 * t5608 - t230 * t5601;
    (t5601, t5605, t5608, t5611)
}
