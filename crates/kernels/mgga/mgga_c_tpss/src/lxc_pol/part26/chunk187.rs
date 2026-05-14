//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 187/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk187<F: Float>(t20: F, t563: F, t12: F, t19: F, t2: F, t27: F, t21: F, t554: F) -> (F, F, F, F, F) {
    let t565 = 4.0 * t20 * t563;
    let t567 = t12 * t19 * t2;
    let t569 = 6.0 * t567 * t27;
    let t570 = t21 * t554;
    let t571 = 1.0 / t570;
    (t565, t567, t569, t570, t571)
}
