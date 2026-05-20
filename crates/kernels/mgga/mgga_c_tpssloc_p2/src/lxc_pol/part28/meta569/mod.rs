//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1848;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta569<F: Float>(t252: F, t4119: F, t22986: F, t6646: F, t829: F, t22690: F, t7520: F, t81573: F, t25249: F, t2684: F, t25324: F, t6562: F, t794: F, t23030: F, t25258: F, t13384: F, t2647: F, t22893: F, t23164: F, t25306: F, t25236: F, t13381: F, t1888: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87130, t87133, t87140, t87150, t87153) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1848::<F>(t252, t4119, t22986, t6646, t829, t22690, t7520, t81573, t25249, t2684, t25324, t6562, t794);
        let (t87155, t87159, t87165, t87171, t87174) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1849::<F>(t23030, t25258, t13384, t22986, t2647, t6646, t22893, t23164, t25306, t25236, t13381, t1888);
    (t87130, t87133, t87140, t87150, t87153, t87155, t87159, t87165, t87171, t87174)
}
