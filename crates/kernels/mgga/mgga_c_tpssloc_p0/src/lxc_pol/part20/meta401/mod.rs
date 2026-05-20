//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1797;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta401<F: Float>(t13748: F, t973: F, t1611: F, t3088: F, t1036: F, t4617: F, t1023: F, t4347: F, t3071: F, t10422: F, t4574: F, t3070: F, t1597: F, t4509: F, t10237: F, t10189: F, t344: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13750, t13751, t13758, t13761, t13762, t13765, t13767) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1797::<F>(t13748, t973, t1611, t3088, t1036, t4617, t1023, t4347, t3071, t10422, t4574, t3070);
        let (t13769, t13770, t13779) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1798::<F>(t1597, t4509, t10237, t10189, t344);
    (t13750, t13751, t13758, t13761, t13762, t13765, t13767, t13769, t13770, t13779)
}
