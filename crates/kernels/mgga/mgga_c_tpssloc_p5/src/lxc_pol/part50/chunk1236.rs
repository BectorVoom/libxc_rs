//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1236/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1236<F: Float>(t1983: F, t24990: F, t31047: F, t26103: F, t7468: F, t26003: F, t6517: F, t24999: F, t6535: F, t31051: F, t4028: F, t26114: F, t8323: F) -> (F, F, F, F, F, F) {
    let t120044 = F::cast_from(3.0_f64) * t1983 * t31047 * t24990;
    let t120045 = t26103 * t7468;
    let t120047 = t6517 * t26003;
    let t120049 = t24999 * t6535;
    let t120051 = t4028 * t31051;
    let t120053 = t26114 * t8323;
    (t120044, t120045, t120047, t120049, t120051, t120053)
}
