//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1432/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1432<F: Float>(t43776: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43833: F, t43835: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F) -> F {
    let t44249 = F::cast_from(0.16979925925925925926e1_f64) * t43776;
    let t44258 = F::new(0.62517e0) * t43759 - F::cast_from(0.10805407407407407407e0_f64) * t43766 + F::cast_from(0.27785333333333333333e0_f64) * t43768 - F::new(0.166712e1) * t43770 + F::cast_from(0.27785333333333333334e0_f64) * t43773 + t44249 + F::new(0.6311625e0) * t43833 + F::cast_from(0.55570666666666666668e0_f64) * t43835 - F::new(0.166712e1) * t43837 - F::cast_from(0.27785333333333333333e0_f64) * t43839 + F::cast_from(0.55570666666666666666e0_f64) * t43842 - F::new(0.125034e1) * t43845 + F::new(0.250068e1) * t43848 + F::new(0.104195e0) * t43851;
    t44258
}
