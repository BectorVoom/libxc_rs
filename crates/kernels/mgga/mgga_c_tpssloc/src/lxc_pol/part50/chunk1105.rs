//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1105/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1105<F: Float>(t26003: F, t6517: F, t24999: F, t6535: F, t31051: F, t4028: F, t26114: F, t8323: F, t26179: F, t31069: F, t7458: F, t120019: F, t120020: F, t120022: F, t120027: F, t120029: F, t120040: F, t120044: F, t120045: F, t1458: F, t2314: F, t24983: F, t30989: F, t32656: F, t4034: F, t4072: F, t652: F, t8439: F) -> (F,) {
    let t120047 = t6517 * t26003;
    let t120049 = t24999 * t6535;
    let t120051 = t4028 * t31051;
    let t120053 = t26114 * t8323;
    let t120055 = t26179 * t8323;
    let t120057 = t7458 * t31069;
    let t120059 = -2.0 * t1458 * t30989 * t652 - 2.0 * t4072 * t652 * t8439 - 2.0 * t2314 * t32656 - 4.0 * t24983 * t6517 - 2.0 * t32656 * t4034 - t120019 - 4.0 * t120020 - 4.0 * t120022 - 4.0 * t120027 - 4.0 * t120029 - 4.0 * t120040 + t120044 - 4.0 * t120045 - 4.0 * t120047 - 4.0 * t120049 - 4.0 * t120051 - 4.0 * t120053 - 4.0 * t120055 - 4.0 * t120057;
    (t120059,)
}
