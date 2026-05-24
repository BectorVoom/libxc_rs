//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1091/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1091<F: Float>(t11880: F, t11885: F, t11890: F, t11896: F, t11899: F, t11904: F, t11908: F, t11938: F, t11940: F, t11941: F, t11943: F, t11952: F, t9221: F, t9223: F, t9226: F, t9228: F, t9243: F) -> F {
    let t11954 = -t9243 + F::new(8.0) / F::new(27.0) * t9221 + F::new(2.0) / F::new(27.0) * t9223 - F::new(2.0) / F::new(9.0) * t9226 - t9228 / F::new(9.0) + F::new(4.0) / F::new(27.0) * t11938 + t11940 - t11941 - t11943 + F::new(10.0) / F::new(27.0) * t11880 - F::new(4.0) / F::new(3.0) * t11885 - F::new(4.0) / F::new(9.0) * t11890 - F::new(2.0) / F::new(9.0) * t11896 + F::new(2.0) * t11899 + F::new(4.0) / F::new(3.0) * t11904 + F::new(2.0) / F::new(3.0) * t11908 + t11952 / F::new(3.0);
    t11954
}
