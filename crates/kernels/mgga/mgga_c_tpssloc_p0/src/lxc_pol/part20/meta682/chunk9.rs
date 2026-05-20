//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2583/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2583<F: Float>(t51039: F, t51051: F, t43859: F, t43861: F, t43863: F, t50968: F, t50970: F, t50972: F, t50976: F, t50978: F, t50987: F, t50990: F, t51034: F, t51037: F, t51041: F, t51043: F, t51046: F, t51049: F, t51053: F, t51056: F) -> F {
    let t52339 = F::new(10.0) / F::new(9.0) * t51039;
    let t52343 = F::new(5.0) / F::new(27.0) * t51051;
    let t52345 = F::new(40.0) / F::new(27.0) * t43859 - F::new(5.0) / F::new(9.0) * t43861 - F::new(10.0) / F::new(9.0) * t43863 - F::new(2.0) / F::new(9.0) * t50968 - t50970 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t50972 + F::new(14.0) / F::new(81.0) * t50976 + F::new(4.0) / F::new(27.0) * t50978 - F::new(2.0) / F::new(9.0) * t50987 - F::new(8.0) / F::new(9.0) * t50990 - F::new(2.0) / F::new(9.0) * t51034 + t51037 - t52339 + F::new(2.0) / F::new(3.0) * t51041 + F::new(2.0) * t51043 + t51046 / F::new(6.0) + t51049 + t52343 + F::new(4.0) / F::new(3.0) * t51053 - t51056;
    t52345
}
