//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 844/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk844<F: Float>(t254: F, t563: F, t12020: F, t2015: F, t1887: F, t22839: F, t12461: F, t2094: F, t193: F, t200: F, t2056: F, t10109: F, t2053: F) -> (F, F, F, F, F, F) {
    let t26224 = t563 * t254;
    let t26225 = t12020 * t2015;
    let t26331 = t22839 * t1887;
    let t26558 = t2094 * t12461;
    let t26563 = t193 * t200 * t2056;
    let t26728 = t10109 * t2053;
    (t26224, t26225, t26331, t26558, t26563, t26728)
}
