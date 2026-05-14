//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 322/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk322<F: Float>(t1042: F, t1043: F, t1024: F, t1011: F, t1017: F, t417: F) -> (F, F, F, F, F, F) {
    let t1044 = t1042 * t1043;
    let t1046 = 1.0 * t1024 * t1044;
    let t1047 = 0.17123333333333333333e-1 * t1011;
    let t1049 = -t1047 + 0.17123333333333333333e-1 * t1017;
    let t1052 = t417 * t417;
    let t1053 = 1.0 / t1052;
    (t1044, t1046, t1047, t1049, t1052, t1053)
}
