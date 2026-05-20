//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1725;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1726;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1727;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta454<F: Float>(t1887: F, t6581: F, t6624: F, t814: F, t2627: F, t6604: F, t6579: F, t6649: F, t1902: F, t1879: F, t22715: F, t1906: F, t6652: F, t794: F, t6562: F, t6547: F, t6653: F, t22723: F, t6561: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t22986 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1725::<F>(t1887, t6581);
        let (t22992, t22996) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1726::<F>(t6624, t814, t2627, t6604);
        let (t23002, t23008, t23012) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1727::<F>(t6579, t6649, t1902, t2627, t1879, t22715);
        let (t23014, t23025, t23026, t23028, t23030) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1728::<F>(t1906, t23012, t6652, t794, t6562, t6547, t6653, t22723, t6561);
    (t22986, t22992, t22996, t23002, t23008, t23012, t23014, t23025, t23026, t23028, t23030)
}
