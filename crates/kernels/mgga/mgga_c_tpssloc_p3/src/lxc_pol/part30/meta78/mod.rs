//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta78 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk515;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta78<F: Float>(t1539: F, t882: F, t123: F, t881: F, t291: F, t880: F, t894: F, t901: F, t908: F, t136: F, t899: F, t907: F, t913: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1540, t1541, t1543, t1545, t1547) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk515::<F>(t1539, t882, t123, t881, t291, t880);
        let (t1548, t1551, t1553, t1554, t1556, t1557) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk516::<F>(t1547, t894, t901, t1539, t908, t136, t1541, t899, t907, t913);
    (t1540, t1541, t1543, t1545, t1547, t1548, t1551, t1553, t1554, t1556, t1557)
}
