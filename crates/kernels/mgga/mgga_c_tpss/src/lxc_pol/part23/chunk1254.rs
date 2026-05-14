//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1254/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1254<F: Float>(t6013: F, t9668: F, t9672: F, t9676: F, t6005: F, t8550: F, t9605: F, t6007: F, t9542: F, t6002: F, t9657: F, t9660: F, t9537: F, t19090: F, t9529: F, t19084: F, t9562: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t63273 = t6013 * t9668;
    let t63275 = t6013 * t9672;
    let t63277 = t6013 * t9676;
    let t63282 = t8550 * t6005 * t9605;
    let t63285 = t6007 * t9542;
    let t63292 = t6002 * t9657;
    let t63294 = t6002 * t9660;
    let t63296 = t6007 * t9537;
    let t63298 = t19090 * t9529;
    let t63300 = t19084 * t9562;
    (t63273, t63275, t63277, t63282, t63285, t63292, t63294, t63296, t63298, t63300)
}
