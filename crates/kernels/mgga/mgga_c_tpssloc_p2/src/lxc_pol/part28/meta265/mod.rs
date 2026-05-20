//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta265<F: Float>(t6889: F, t7700: F, t1985: F, t1811: F, t6916: F, t1799: F, t236: F, t1998: F, t6926: F, t1339: F, t1825: F, t6936: F) -> (F, F, F, F, F, F, F, F) {
        let (t7701, t7702, t7706, t7708, t7709, t7710, t7712, t7713) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1135::<F>(t6889, t7700, t1985, t1811, t6916, t1799, t236, t1998, t6926, t1339, t1825, t6936);
    (t7701, t7702, t7706, t7708, t7709, t7710, t7712, t7713)
}
