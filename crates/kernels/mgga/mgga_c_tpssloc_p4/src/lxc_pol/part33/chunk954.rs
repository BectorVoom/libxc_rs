//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 954/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk954<F: Float>(t20811: F, t20812: F, t20821: F, t20832: F, t225: F, t20756: F, t9946: F, t4226: F, t5544: F, t20800: F, t824: F, t1504: F, t1506: F, t228: F, t230: F, t4225: F, t5601: F, t5605: F, t5608: F) -> F {
    let t20835 = (t20811 + t20812 + t20821 + t20832) * t225;
    let t20843 = t9946 * t20756;
    let t20846 = t4226 * t5544;
    let t20849 = t824 * t20800;
    let t20852 = -F::new(36.0) * t1504 * t5605 + F::new(9.0) * t1504 * t5608 + F::new(9.0) * t1506 * t5601 - t20835 * t230 + F::new(60.0) * t20843 * t228 - F::new(36.0) * t20846 * t4225 + F::new(3.0) * t20849 * t228;
    t20852
}
