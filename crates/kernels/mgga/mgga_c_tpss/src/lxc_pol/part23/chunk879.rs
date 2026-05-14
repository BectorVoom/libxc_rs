//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 879/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk879<F: Float>(t7612: F, t2: F, t22: F, t106: F, t107: F, t10: F, t555: F, t551: F, t15: F, t563: F, t11: F, t1958: F, t27: F, t559: F, t20: F, t571: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7613 = 1.0 / t7612;
    let t7622 = t2 * t22;
    let t7628 = t107 * t106;
    let t7629 = 1.0 / t7628;
    let t7651 = t10 * t555;
    let t7653 = t551 * t22;
    let t7656 = 24.0 * t15 * t563;
    let t7657 = t11 * t2;
    let t7659 = 24.0 * t7657 * t22;
    let t7660 = t1958 * t563;
    let t7662 = t559 * t27;
    let t7665 = 120.0 * t20 * t571;
    (t7613, t7622, t7629, t7651, t7653, t7656, t7659, t7660, t7662, t7665)
}
