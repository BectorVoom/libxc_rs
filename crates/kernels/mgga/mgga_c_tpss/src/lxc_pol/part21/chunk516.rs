//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 516/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk516<F: Float>(t1786: F, t547: F, t1784: F, t548: F, t10: F, t17: F, t551: F, t555: F, t15: F, t22: F, t11: F, t14: F, t559: F, t563: F, t20: F, t27: F) -> (F, F, F, F, F, F, F, F) {
    let t1788 = 3.0 * t547 * t1786;
    let t1789 = t1784 * t548 + t1788;
    let t1953 = 2.0 * t10 * t17;
    let t1955 = 8.0 * t551 * t555;
    let t1957 = 6.0 * t15 * t22;
    let t1958 = t11 * t14;
    let t1960 = 12.0 * t1958 * t22;
    let t1962 = 32.0 * t559 * t563;
    let t1964 = 20.0 * t20 * t27;
    (t1789, t1953, t1955, t1957, t1958, t1960, t1962, t1964)
}
