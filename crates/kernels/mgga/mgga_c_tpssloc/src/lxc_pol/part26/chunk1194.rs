//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1194/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1194<F: Float>(t1404: F, t7415: F, t2174: F, t3931: F, t24954: F, t580: F, t111: F, t112: F, t24542: F, t2312: F, t2314: F, t2363: F, t2364: F, t24552: F, t24932: F, t4034: F, t652: F, t672: F, t7408: F, t80609: F, t80611: F, t80614: F, t80617: F, t80620: F, t80622: F, t80625: F, t80627: F, t80629: F, t80633: F, t80635: F, t80637: F, t81410: F, t81412: F) -> (F, F, F, F, F, F, F) {
    let t85405 = t7415 * t1404;
    let t85407 = t3931 * t2174;
    let t85412 = t24954 * t580;
    let t85416 = t7415 * t111;
    let t85423 = t24954 * t112;
    let t85428 = t24542 * t111;
    let t85442 = -6.0 * t2363 * t652 * t7408 - 3.0 * t2312 * t7408 - 6.0 * t2314 * t24552 - 6.0 * t2364 * t24932 - 6.0 * t24552 * t4034 - 6.0 * t672 * t85428 + t80609 - t80611 + t80614 - t80617 - t80620 - t80622 - t80625 - t80627 - t80629 + t80633 + t80635 + t80637 + t81410 - t81412;
    (t85405, t85407, t85412, t85416, t85423, t85428, t85442)
}
