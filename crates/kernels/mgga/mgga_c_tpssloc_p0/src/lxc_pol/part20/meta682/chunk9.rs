//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2583/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2583<F: Float>(t51039: F, t51051: F, t43859: F, t43861: F, t43863: F, t50968: F, t50970: F, t50972: F, t50976: F, t50978: F, t50987: F, t50990: F, t51034: F, t51037: F, t51041: F, t51043: F, t51046: F, t51049: F, t51053: F, t51056: F) -> F {
    let t52339 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t51039;
    let t52343 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t51051;
    let t52345 = F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t43859 - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t43861 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t43863 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t50968 - t50970 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t50972 + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t50976 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t50978 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t50987 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t50990 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t51034 + t51037 - t52339 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t51041 + F::cast_from(2.0_f64) * t51043 + t51046 / F::cast_from(6.0_f64) + t51049 + t52343 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t51053 - t51056;
    t52345
}
