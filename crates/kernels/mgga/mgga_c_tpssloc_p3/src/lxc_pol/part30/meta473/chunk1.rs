//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1768/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1768<F: Float>(t112: F, t7002: F, t111: F, t2022: F, t1976: F, t4072: F, t671: F, t7670: F, t191: F, t192: F, t5118: F, t2020: F) -> (F, F, F, F, F, F) {
    let t23877 = t7002 * t112;
    let t23880 = t2022 * t111;
    let t24980 = t1976 * t4072;
    let t24983 = t7670 * t671;
    let t24987 = t5118 * t191 * t192;
    let t24988 = t24987 * t2020;
    (t23877, t23880, t24980, t24983, t24987, t24988)
}
