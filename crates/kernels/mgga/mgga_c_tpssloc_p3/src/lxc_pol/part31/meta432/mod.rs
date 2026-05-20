//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1564;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1565;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta432<F: Float>(t2627: F, t6604: F, t6579: F, t6649: F, t1879: F, t22715: F, t1906: F, t6652: F, t794: F, t6562: F, t6547: F, t6653: F, t22723: F, t6561: F) -> (F, F, F, F, F, F, F, F) {
        let t22996 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1564::<F>(t2627, t6604);
        let (t23003, t23012) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1565::<F>(t6579, t6649, t1879, t22715);
        let (t23013, t23025, t23026, t23029, t23030) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1566::<F>(t1906, t23012, t6652, t794, t6562, t6547, t6653, t22723, t6561);
    (t22996, t23003, t23012, t23013, t23025, t23026, t23029, t23030)
}
