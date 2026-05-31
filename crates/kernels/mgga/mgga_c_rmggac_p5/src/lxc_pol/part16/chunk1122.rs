//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1122/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1122<F: Float>(t36674: F, t43874: F, t43877: F, t47594: F, t47596: F, t47598: F, t47600: F, t47602: F, t47607: F, t47612: F, t47616: F, t47621: F, t47623: F, t47629: F, t47634: F, t47639: F, t5144: F, t739: F, t9530: F) -> F {
    let t49277 = F::cast_from(0.23948483403727617128e0_f64) * t739 * t9530 * t5144 - F::cast_from(0.30487649791575028312e-3_f64) * t36674 - F::cast_from(0.14365552463988020798e-3_f64) * t47594 + F::cast_from(0.3405167991463827152e-4_f64) * t47596 - F::cast_from(0.5107751987195740728e-4_f64) * t47598 + F::cast_from(0.5107751987195740728e-4_f64) * t47600 + F::cast_from(0.1702583995731913576e-4_f64) * t47602 - F::cast_from(0.1702583995731913576e-4_f64) * t47607 + F::cast_from(0.14365552463988020798e-3_f64) * t47612 + t43874 + F::cast_from(2.0_f64) * t43877 + F::cast_from(0.18183107769496894487e-1_f64) * t47616 - F::cast_from(0.5107751987195740728e-4_f64) * t47621 - F::cast_from(0.85129199786595678799e-5_f64) * t47623 + F::cast_from(0.1064114997332445985e-4_f64) * t47629 + F::cast_from(0.1702583995731913576e-4_f64) * t47634 - F::cast_from(0.2553875993597870364e-4_f64) * t47639;
    t49277
}
