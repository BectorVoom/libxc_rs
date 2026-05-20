//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1923;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta611<F: Float>(t22633: F, t26421: F, t3856: F, t6976: F, t26462: F, t6914: F, t22705: F, t26414: F, t81228: F, t26415: F, t81159: F, t3851: F, t26418: F, t7736: F, t80854: F, t81064: F, t22704: F, t26410: F, t26432: F, t6897: F, t794: F, t22642: F, t22690: F, t26395: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90933, t90956, t90961, t90963, t90968) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1923::<F>(t22633, t26421, t3856, t6976, t26462, t6914, t22705, t26414, t81228, t26415, t81159, t3851);
        let (t90970, t90980, t90983, t90987, t90993) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1924::<F>(t26418, t6914, t7736, t80854, t81064, t22704, t22705, t26410, t26432, t6897, t794, t22642, t22690, t26395);
    (t90933, t90956, t90961, t90963, t90968, t90970, t90980, t90983, t90987, t90993)
}
