//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1356/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1356<F: Float>(t120694: F, t26161: F, t26558: F, t31670: F, t7685: F, t33363: F, t6997: F, t1873: F, t4025: F, t2040: F, t33334: F, t532: F) -> (F, F, F, F, F, F) {
    let t120944 = F::new(2.0) * t26161 * t26558 * t120694;
    let t120947 = t7685 * t31670;
    let t120948 = t33363 * t6997;
    let t120952 = t4025 * t1873;
    let t120954 = F::new(2.0) * t120952 * t2040;
    let t120955 = t532 * t33334;
    (t120944, t120947, t120948, t120952, t120954, t120955)
}
