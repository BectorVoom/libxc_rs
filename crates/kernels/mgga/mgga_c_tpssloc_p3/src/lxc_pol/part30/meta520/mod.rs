//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta520<F: Float>(t5: F, t26054: F, t26095: F, t112: F, t1868: F, t671: F, t12725: F, t1873: F, t19456: F, t4028: F, t6534: F, t1458: F, t649: F) -> (F, F, F, F, F, F, F) {
        let (t26097, t26098, t26103, t26109, t26111, t26113, t26114) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1853::<F>(t5, t26054, t26095, t112, t1868, t671, t12725, t1873, t19456, t4028, t6534, t1458, t649);
    (t26097, t26098, t26103, t26109, t26111, t26113, t26114)
}
