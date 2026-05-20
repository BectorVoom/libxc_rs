//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta527<F: Float>(t1068: F, t1637: F, t1458: F, t649: F, t4072: F, t88: F, t89: F, t254: F, t563: F, t1351: F, t16311: F, t16306: F, t550: F) -> (F, F, F, F, F, F, F) {
        let (t25845, t26114, t26117, t26179, t26224, t26318, t26322) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1997::<F>(t1068, t1637, t1458, t649, t4072, t88, t89, t254, t563, t1351, t16311, t16306, t550);
    (t25845, t26114, t26117, t26179, t26224, t26318, t26322)
}
