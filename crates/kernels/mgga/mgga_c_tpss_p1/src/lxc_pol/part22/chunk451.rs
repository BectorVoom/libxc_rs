//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 451/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk451<F: Float>(t1211: F, t1213: F, t1222: F, t1241: F, t1244: F, t1630: F, t1642: F, t1646: F) -> F {
    let t1649 = -t1211 - t1213 * t1630 / F::new(48.0) - t1222 * t1642 / F::new(3072.0) - t1241 - t1244 * t1646 / F::new(768.0);
    t1649
}
