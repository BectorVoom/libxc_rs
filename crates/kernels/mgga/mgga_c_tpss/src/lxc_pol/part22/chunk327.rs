//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 327/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk327<F: Float>(t1061: F, t1062: F, t1011: F, t1017: F) -> (F, F, F) {
    let t1063 = t1061 * t1062;
    let t1066 = F::cast_from(0.92708333333333333333e-2_f64) * t1011;
    let t1068 = -t1066 + F::cast_from(0.92708333333333333333e-2_f64) * t1017;
    (t1063, t1066, t1068)
}
