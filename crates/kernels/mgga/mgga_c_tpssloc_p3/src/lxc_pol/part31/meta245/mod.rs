//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1032;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1033;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1034;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta245<F: Float>(t6553: F, t6572: F, t1880: F, t1887: F, t206: F, t6546: F, t1878: F, t229: F, t805: F, t1891: F, t2230: F, t213: F, t1895: F, t202: F, t243: F, t598: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6573, t6574, t6579) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1032::<F>(t6553, t6572, t1880, t1887, t206, t6546);
        let t6581 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1033::<F>(t1878, t229);
        let (t6582, t6584, t6586, t6589) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1034::<F>(t6581, t805, t1891, t2230, t213, t1895, t202, t243);
        let (t6590, t6591) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1035::<F>(t598, t6589, t213);
    (t6573, t6574, t6579, t6581, t6582, t6584, t6586, t6589, t6590, t6591)
}
