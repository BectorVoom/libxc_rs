//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2500/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2500<F: Float>(t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50919: F, t50921: F, t50926: F, t50931: F, t50934: F, t50937: F, t50940: F) -> F {
    let t50942 = -F::cast_from(0.12077e1_f64) * t50903 - F::cast_from(0.60385e0_f64) * t50905 - F::cast_from(0.181155e1_f64) * t50907 + F::cast_from(0.10064166666666666666e1_f64) * t50912 + F::cast_from(0.40256666666666666666e1_f64) * t50917 - F::cast_from(0.26837777777777777778e0_f64) * t50919 - F::cast_from(0.33547222222222222222e0_f64) * t50921 - F::cast_from(0.89459259259259259259e0_f64) * t50926 + F::cast_from(0.181155e1_f64) * t50931 + F::cast_from(0.181155e1_f64) * t50934 + F::cast_from(0.543465e1_f64) * t50937 + F::cast_from(0.60385e0_f64) * t50940;
    t50942
}
