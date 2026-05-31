//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2860/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2860<F: Float>(t41658: F, t41675: F, t41684: F, t59655: F, t59657: F, t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t59680: F, t59684: F) -> F {
    let t59802 = -F::cast_from(0.50735802469135802469e-2_f64) * t41658 + F::cast_from(0.15220740740740740741e-1_f64) * t41675 + F::cast_from(0.35515061728395061728e-1_f64) * t41684 - F::cast_from(0.41096e0_f64) * t59655 - F::cast_from(0.50735802469135802467e-2_f64) * t59657 + F::cast_from(0.41096e0_f64) * t59661 - F::cast_from(0.2283111111111111111e-1_f64) * t59663 + F::cast_from(0.76103703703703703701e-2_f64) * t59665 - F::cast_from(0.2283111111111111111e-1_f64) * t59670 - F::cast_from(0.11415555555555555555e-1_f64) * t59674 - F::cast_from(0.2283111111111111111e-1_f64) * t59678 + F::cast_from(0.11415555555555555555e-1_f64) * t59680 - F::cast_from(0.17123333333333333333e-1_f64) * t59684;
    t59802
}
