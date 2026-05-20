//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1543;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1544;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1545;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta423<F: Float>(t22705: F, t6978: F, t22704: F, t154: F, t2558: F, t1984: F, t2010: F, t591: F, t6896: F) -> (F, F, F, F, F, F, F) {
        let (t22706, t22707, t22715) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1543::<F>(t22705, t6978, t22704, t154, t2558);
        let t22716 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1544::<F>(t1984, t22715);
        let (t22717, t22723) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1545::<F>(t2010, t22716, t154, t591);
        let t22724 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1546::<F>(t22723, t6896);
    (t22706, t22707, t22715, t22716, t22717, t22723, t22724)
}
