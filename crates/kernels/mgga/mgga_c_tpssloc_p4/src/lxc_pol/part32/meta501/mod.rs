//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1821;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1822;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta501<F: Float>(t4028: F, t6534: F, t1458: F, t649: F, t1873: F, t4072: F, t88: F, t7676: F, t2314: F, t7467: F, t5113: F, t1453: F, t22470: F, t666: F, t109: F, t22473: F, t4067: F, t6530: F, t22469: F, t22471: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26113, t26114) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1821::<F>(t4028, t6534, t1458, t649);
        let (t26116, t26117, t26119, t26121, t26123, t26125, t26127, t26129) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1822::<F>(t1873, t26114, t4072, t88, t6534, t7676, t2314, t7467, t5113, t1453, t22470, t666);
        let t26135 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1823::<F>(t109, t22473, t26129, t4067, t6530, t22469, t22471, t26127);
    (t26113, t26114, t26116, t26117, t26119, t26121, t26123, t26125, t26127, t26129, t26135)
}
