//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1897;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta522<F: Float>(t26066: F, t72: F, t1410: F, t2235: F, t3961: F, t605: F, t3967: F, t1433: F, t645: F, t12725: F, t1873: F, t19456: F, t4028: F, t6534: F, t1458: F, t649: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26067, t26070, t26073, t26076, t26090, t26109, t26111) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1897::<F>(t26066, t72, t1410, t2235, t3961, t605, t3967, t1433, t645, t12725, t1873, t19456);
        let (t26113, t26114) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1898::<F>(t4028, t6534, t1458, t649);
    (t26067, t26070, t26073, t26076, t26090, t26109, t26111, t26113, t26114)
}
