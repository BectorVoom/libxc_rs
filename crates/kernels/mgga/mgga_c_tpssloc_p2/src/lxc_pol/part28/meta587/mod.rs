//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1879;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta587<F: Float>(t22986: F, t6646: F, t829: F, t87111: F, t25273: F, t6579: F, t244: F, t268: F, t6559: F, t25250: F, t87202: F, t25316: F, t82038: F, t1888: F, t232: F, t47439: F, t23110: F, t23185: F, t25272: F, t25325: F, t6547: F, t1880: F, t7488: F, t82124: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t87705, t87709, t87712, t87714, t87718) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1879::<F>(t22986, t6646, t829, t87111, t25273, t6579, t244, t268, t6559, t25250, t87202, t25316, t82038);
        let (t87726, t87729, t87733, t87746) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1880::<F>(t1888, t232, t47439, t6646, t23110, t23185, t25272, t25325, t6547, t1880, t7488, t82124);
    (t87705, t87709, t87712, t87714, t87718, t87726, t87729, t87733, t87746)
}
