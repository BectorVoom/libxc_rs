//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1394;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta334<F: Float>(t1229: F, t676: F, t1090: F, t248: F, t1227: F, t3536: F, t3572: F, t3252: F, t3521: F, t3248: F, t1009: F, t3481: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11789, t11791, t11792, t11794, t11797, t11798, t11801, t11802, t11812) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1394::<F>(t1229, t676, t1090, t248, t1227, t3536, t3572, t3252, t3521, t3248, t1009, t3481);
    (t11789, t11791, t11792, t11794, t11797, t11798, t11801, t11802, t11812)
}
