//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta597<F: Float>(t254: F, t563: F, t1215: F, t1409: F, t460: F, t4928: F, t492: F, t64: F, t9365: F, t1444: F, t659: F, t1449: F, t662: F) -> (F, F, F, F, F, F, F) {
        let (t26224, t27524, t27654, t27784, t29903, t30171, t30307) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2348::<F>(t254, t563, t1215, t1409, t460, t4928, t492, t64, t9365, t1444, t659, t1449, t662);
    (t26224, t27524, t27654, t27784, t29903, t30171, t30307)
}
