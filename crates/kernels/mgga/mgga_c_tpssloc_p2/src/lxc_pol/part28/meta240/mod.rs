//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1052;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1053;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1054;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta240<F: Float>(t1882: F, t794: F, t6562: F, t225: F, t258: F, t852: F, t214: F, t1880: F, t857: F, t865: F, t6553: F, t1887: F, t206: F, t6546: F, t1878: F, t229: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6563, t6564, t6567, t6568, t6569, t6571) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1052::<F>(t1882, t794, t6562, t225, t258, t852, t214, t1880, t857);
        let t6572 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1053::<F>(t6571, t865);
        let (t6573, t6574, t6579) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1054::<F>(t6553, t6572, t1880, t1887, t206, t6546);
        let t6581 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1055::<F>(t1878, t229);
    (t6563, t6564, t6567, t6568, t6569, t6571, t6572, t6573, t6574, t6579, t6581)
}
