//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 880/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk880<F: Float>(t761: F, t9919: F, t2531: F, t2535: F, t32: F, t717: F, t2617: F, t2629: F, t813: F, t236: F, t232: F, t2632: F) -> (F, F, F, F, F, F, F) {
    let t9921 = F::cast_from(0.35089341735807877242e1_f64) * t761 * t9919;
    let t9922 = t2531 * t2535;
    let t9929 = t32 * t717;
    let t9967 = t2617 * t2629;
    let t9970 = t813 * t813;
    let t9971 = F::new(1.0) / t9970;
    let t9972 = t9971 * t236;
    let t9975 = t2632 * t232;
    (t9921, t9922, t9929, t9967, t9971, t9972, t9975)
}
