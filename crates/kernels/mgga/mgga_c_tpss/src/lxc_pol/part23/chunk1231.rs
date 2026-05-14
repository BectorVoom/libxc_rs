//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1231/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1231<F: Float>(t20913: F, t6034: F, t1885: F, t20853: F, t452: F, t1884: F, t19118: F, t19129: F, t19143: F, t20883: F, t20887: F, t20893: F, t20897: F, t20900: F, t20904: F, t20906: F, t20910: F, t6022: F, t6024: F, t6031: F, t6522: F, t6525: F) -> (F, F, F) {
    let t20914 = t20913 * t6034;
    let t20917 = t1885 * t452 * t20853;
    let t20919 = -t1884 * t20917 - t19118 * t6522 + 2.0 * t19129 * t20893 - 2.0 * t19143 * t20897 + t19143 * t20906 + 2.0 * t20883 * t6024 + 2.0 * t20887 * t6024 - t20900 * t6031 - t20904 * t6031 + 2.0 * t20910 * t6024 - t20914 * t6031 - t6022 * t6525;
    (t20914, t20917, t20919)
}
