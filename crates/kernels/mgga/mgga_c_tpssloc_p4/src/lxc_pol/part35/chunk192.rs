//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 192/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk192<F: Float>(t539: F, t562: F, t553: F, t544: F) -> (F, F, F, F) {
    let t563 = t539 * t562;
    let t564 = t553 * t562;
    let t566 = t544 * t564 + F::cast_from(1.0_f64);
    let t567 = F::cast_from(1.0_f64) / t566;
    (t563, t564, t566, t567)
}
