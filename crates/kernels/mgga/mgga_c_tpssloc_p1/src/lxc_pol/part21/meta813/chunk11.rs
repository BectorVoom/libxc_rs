//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2868/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2868<F: Float>(t10813: F, t5758: F, t17195: F, t2837: F, t2841: F, t5689: F, t2845: F, t17471: F, t923: F, t1557: F, t49483: F, t13515: F, t4396: F) -> (F, F, F, F, F, F) {
    let t59941 = t5758 * t10813;
    let t59958 = F::new(1.0) * t17195 * t2837;
    let t59959 = t5689 * t2841;
    let t59961 = F::cast_from(0.16081979498692535067e2_f64) * t59959 * t2845;
    let t59962 = t17471 * t923;
    let t59966 = F::new(2.0) * t49483 * t1557;
    let t59968 = F::new(4.0) * t13515 * t4396;
    (t59941, t59958, t59961, t59962, t59966, t59968)
}
