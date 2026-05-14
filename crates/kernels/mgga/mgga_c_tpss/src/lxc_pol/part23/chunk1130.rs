//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1130/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1130<F: Float>(t10456: F, t1689: F, t2056: F, t5522: F, t2116: F, t30: F, t1712: F, t198: F, t206: F) -> (F, F, F, F) {
    let t17913 = 4.0 * t10456 * t1689;
    let t17915 = 4.0 * t2056 * t5522;
    let t17921 = t30 * t2116;
    let t17929 = t198 * t206 * t1712;
    (t17913, t17915, t17921, t17929)
}
