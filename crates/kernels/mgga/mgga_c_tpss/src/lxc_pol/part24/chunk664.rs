//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 664/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk664<F: Float>(t4016: F, t981: F, t1483: F, t2771: F, t373: F, t3988: F, t3990: F, t3994: F, t978: F, t991: F, t198: F, t330: F) -> (F, F, F) {
    let t4017 = t981 * t4016;
    let t4019 = -t1483 * t2771 + t373 * t3988 - t3990 * t991 + 2.0 * t3994 * t978 - t4017 * t978;
    let t4023 = t198 * t330;
    (t4017, t4019, t4023)
}
