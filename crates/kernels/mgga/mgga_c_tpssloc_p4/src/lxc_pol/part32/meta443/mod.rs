//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1693;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1694;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1695;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1696;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1697;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta443<F: Float>(t532: F, t6995: F, t2018: F, t531: F, t1887: F, t6916: F, t213: F, t225: F, t562: F, t154: F, t835: F, t3748: F, t212: F, t6890: F, t6911: F, t1372: F, t214: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22591, t22595, t22633) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1693::<F>(t532, t6995, t2018, t531, t1887, t6916);
        let t22635 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1694::<F>(t213, t225, t562);
        let t22641 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1695::<F>(t154, t835);
        let t22642 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1696::<F>(t22641, t3748);
        let t22643 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1697::<F>(t212, t562);
        let (t22644, t22646, t22656, t22666) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1698::<F>(t22643, t6890, t22642, t225, t6911, t1372, t214);
    (t22591, t22595, t22633, t22635, t22641, t22642, t22643, t22644, t22646, t22656, t22666)
}
