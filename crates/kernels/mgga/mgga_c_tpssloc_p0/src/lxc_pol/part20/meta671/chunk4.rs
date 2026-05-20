//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2525/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2525<F: Float>(t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50919: F, t50921: F, t50926: F, t50931: F, t50934: F, t50937: F, t50940: F) -> F {
    let t51186 = -F::cast_from(0.11958666666666666667e1_f64) * t50903 - F::cast_from(0.59793333333333333333e0_f64) * t50905 - F::new(0.17938e1) * t50907 + F::cast_from(0.99655555555555555555e0_f64) * t50912 + F::cast_from(0.39862222222222222223e1_f64) * t50917 - F::cast_from(0.26574814814814814816e0_f64) * t50919 - F::cast_from(0.33218518518518518519e0_f64) * t50921 - F::cast_from(0.88582716049382716048e0_f64) * t50926 + F::new(0.17938e1) * t50931 + F::new(0.17938e1) * t50934 + F::cast_from(0.53814000000000000001e1_f64) * t50937 + F::cast_from(0.59793333333333333334e0_f64) * t50940;
    t51186
}
