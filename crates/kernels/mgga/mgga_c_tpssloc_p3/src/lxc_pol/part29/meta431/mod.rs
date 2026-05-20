//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1725;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta431<F: Float>(t22643: F, t6890: F, t22642: F, t225: F, t3879: F, t567: F, t214: F, t1985: F, t1385: F, t6992: F, t3887: F, t6911: F, t3911: F, t6906: F, t6889: F, t1372: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22644, t22646, t22648, t22649, t22650, t22653, t22656) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1725::<F>(t22643, t6890, t22642, t225, t3879, t567, t214, t1985, t1385, t6992, t3887, t6911);
        let (t22662, t22663, t22664, t22666) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1726::<F>(t3911, t6906, t6889, t1985, t1372, t214);
    (t22644, t22646, t22648, t22649, t22650, t22653, t22656, t22662, t22663, t22664, t22666)
}
