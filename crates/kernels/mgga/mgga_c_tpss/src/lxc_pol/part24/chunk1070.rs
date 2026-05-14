//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1070/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1070<F: Float>(t15199: F, t981: F, t11710: F, t1483: F, t15118: F, t15120: F, t15131: F, t15135: F, t15140: F, t2771: F, t373: F, t3990: F, t3994: F, t4017: F, t5018: F, t5037: F, t978: F, t991: F) -> (F, F) {
    let t15200 = t981 * t15199;
    let t15202 = -2.0 * t11710 * t1483 + t15118 * t373 - t15120 * t991 - 6.0 * t15131 * t978 + 4.0 * t15135 * t978 + 2.0 * t15140 * t978 - t15200 * t978 + 2.0 * t2771 * t5018 - t2771 * t5037 + 4.0 * t3990 * t3994 - 2.0 * t3990 * t4017;
    (t15200, t15202)
}
