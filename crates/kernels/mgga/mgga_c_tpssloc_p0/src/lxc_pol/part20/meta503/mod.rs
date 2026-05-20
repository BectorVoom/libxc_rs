//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta503<F: Float>(t254: F, t563: F, t1351: F, t1834: F, t492: F, t64: F, t9365: F, t1444: F, t659: F, t1449: F, t662: F, t20: F, t60: F) -> (F, F, F, F, F, F, F) {
        let (t26224, t26409, t27784, t29903, t30171, t30307, t32253) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2012::<F>(t254, t563, t1351, t1834, t492, t64, t9365, t1444, t659, t1449, t662, t20, t60);
    (t26224, t26409, t27784, t29903, t30171, t30307, t32253)
}
