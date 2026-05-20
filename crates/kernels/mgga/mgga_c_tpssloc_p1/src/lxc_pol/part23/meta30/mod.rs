//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta30 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk223;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk224;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk225;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta30<F: Float>(t588: F, t15: F, t3: F, t14: F, t2: F, t21: F, t583: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t589, t590, t591) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk223::<F>(t588, t15, t3);
        let t592 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk224::<F>(t14, t591);
        let (t593, t594, t596, t597, t598) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk225::<F>(t592, t14, t2, t21, t15, t583);
    (t589, t590, t591, t592, t593, t594, t596, t597, t598)
}
