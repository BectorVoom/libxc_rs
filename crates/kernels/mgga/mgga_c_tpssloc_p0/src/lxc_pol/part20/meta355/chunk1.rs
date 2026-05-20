//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1670/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1670<F: Float>(t10027: F, t541: F, t12267: F, t1362: F, t3777: F, t3865: F) -> (F, F, F) {
    let t12335 = F::new(455.0) / F::new(1296.0) * t10027 * t541;
    let t12336 = t12267 * t1362;
    let t12339 = t3777 * t3865;
    (t12335, t12336, t12339)
}
