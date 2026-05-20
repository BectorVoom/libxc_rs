//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1147;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1148;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1149;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1150;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta254<F: Float>(t1307: F, t1390: F, t6878: F, t1983: F, t1984: F, t6546: F, t1988: F, t131: F, t209: F, t547: F, t1878: F, t214: F, t562: F, t225: F, t567: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6879, t6880, t6882, t6883) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1147::<F>(t1307, t1390, t6878, t1983, t1984, t6546);
        let (t6885, t6887, t6888) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1148::<F>(t1988, t6883, t131, t209, t547, t1878);
        let t6889 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1149::<F>(t214, t562);
        let t6890 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1150::<F>(t225, t567);
        let t6891 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1151::<F>(t1307, t6890);
    (t6879, t6880, t6882, t6883, t6885, t6887, t6888, t6889, t6890, t6891)
}
