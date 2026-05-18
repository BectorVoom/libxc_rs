//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1337/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1337<F: Float>(t112: F, t24954: F, t111: F, t24542: F, t2312: F, t2314: F, t2363: F, t2364: F, t24552: F, t24932: F, t4034: F, t652: F, t672: F, t7408: F, t80609: F, t80611: F, t80614: F, t80617: F, t80620: F, t80622: F, t80625: F, t80627: F, t80629: F, t80633: F, t80635: F, t80637: F, t81410: F, t81412: F) -> (F, F, F) {
    let t85423 = t24954 * t112;
    let t85428 = t24542 * t111;
    let t85442 = -F::new(6.0) * t2363 * t652 * t7408 - F::new(3.0) * t2312 * t7408 - F::new(6.0) * t2314 * t24552 - F::new(6.0) * t2364 * t24932 - F::new(6.0) * t24552 * t4034 - F::new(6.0) * t672 * t85428 + t80609 - t80611 + t80614 - t80617 - t80620 - t80622 - t80625 - t80627 - t80629 + t80633 + t80635 + t80637 + t81410 - t81412;
    (t85423, t85428, t85442)
}
