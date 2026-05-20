//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk775;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk776;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta142<F: Float>(t1339: F, t835: F, t1336: F, t1354: F, t242: F, t1365: F, t67: F, t246: F, t1307: F, t550: F) -> (F, F, F, F, F, F, F, F) {
        let (t3798, t3799) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk775::<F>(t1339, t835, t1336);
        let (t3800, t3802, t3803, t3804, t3805) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk776::<F>(t1354, t3799, t1339, t242, t1336, t1365, t67, t246);
        let t3807 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk777::<F>(t1307, t550);
    (t3798, t3799, t3800, t3802, t3803, t3804, t3805, t3807)
}
