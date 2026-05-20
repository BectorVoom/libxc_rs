//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1256;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta325<F: Float>(t11998: F, t517: F, t1376: F, t68: F, t225: F, t3753: F, t3880: F, t522: F, t9212: F, t9214: F, t3824: F, t592: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12000, t12019, t12020, t12021, t12030, t12033, t12044, t12045, t12048) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1256::<F>(t11998, t517, t1376, t68, t225, t3753, t3880, t522, t9212, t9214, t3824, t592);
    (t12000, t12019, t12020, t12021, t12030, t12033, t12044, t12045, t12048)
}
