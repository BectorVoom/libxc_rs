//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta453<F: Float>(t23110: F, t232: F, t236: F, t828: F, t23109: F, t1898: F, t2613: F, t249: F, t6609: F, t838: F, t6589: F, t6597: F) -> (F, F, F, F, F, F, F) {
        let (t23113, t23114, t23116, t23117, t23119, t23120, t23121) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1794::<F>(t23110, t232, t236, t828, t23109, t1898, t2613, t249, t6609, t838, t6589, t6597);
    (t23113, t23114, t23116, t23117, t23119, t23120, t23121)
}
