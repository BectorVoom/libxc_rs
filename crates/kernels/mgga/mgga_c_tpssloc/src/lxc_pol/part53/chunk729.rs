//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 729/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk729<F: Float>(t1307: F, t26214: F, t22635: F, t22633: F, t254: F, t563: F, t1827: F, t22765: F, t5234: F, t6944: F, t1354: F, t22756: F, t5289: F, t6945: F, t5310: F, t6952: F) -> (F, F, F, F, F, F, F, F) {
    let t26215 = t26214 * t1307;
    let t26216 = t22635 * t26215;
    let t26217 = t22633 * t26216;
    let t26224 = t563 * t254;
    let t26231 = t22765 * t1827;
    let t26233 = t5234 * t6944;
    let t26234 = t26233 * t1354;
    let t26236 = t22756 * t1827;
    let t26238 = t6945 * t5289;
    let t26240 = t6952 * t5310;
    (t26215, t26217, t26224, t26231, t26234, t26236, t26238, t26240)
}
