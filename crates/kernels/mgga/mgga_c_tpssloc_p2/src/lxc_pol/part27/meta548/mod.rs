//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta548<F: Float>(t1799: F, t567: F, t1307: F, t22635: F, t26331: F, t1377: F, t1385: F, t22633: F, t22674: F, t7700: F, t6897: F, t1842: F, t6992: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26332, t26333, t26334, t26335, t26337, t26338, t26339, t26340, t26344, t26345, t26347) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1981::<F>(t1799, t567, t1307, t22635, t26331, t1377, t1385, t22633, t22674, t7700, t6897, t1842, t6992);
    (t26332, t26333, t26334, t26335, t26337, t26338, t26339, t26340, t26344, t26345, t26347)
}
