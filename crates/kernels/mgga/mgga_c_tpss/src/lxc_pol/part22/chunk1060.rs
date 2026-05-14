//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1060/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1060<F: Float>(t1226: F, t1229: F, t12928: F, t12938: F, t12944: F, t12948: F, t12951: F, t12954: F, t1634: F, t1636: F, t3315: F, t3320: F, t3323: F, t4445: F, t4451: F, t4453: F, t4456: F, t516: F, t518: F) -> (F,) {
    let t12957 = 6.0 * t1226 * t4456 + 6.0 * t1229 * t4445 - t12928 * t518 - 24.0 * t12938 * t4453 + 60.0 * t12944 * t4451 - 24.0 * t12948 * t4451 - 12.0 * t12951 * t4451 + 3.0 * t12954 * t516 - 12.0 * t1634 * t3320 + 3.0 * t1634 * t3323 + 3.0 * t1636 * t3315;
    (t12957,)
}
