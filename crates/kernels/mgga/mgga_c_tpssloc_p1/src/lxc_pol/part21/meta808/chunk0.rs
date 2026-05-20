//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2822/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2822<F: Float>(t2: F, t4324: F, t584: F, t1534: F, t16: F, t17139: F, t14389: F, t48763: F, t41656: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t47738: F) -> (F, F, F, F, F) {
    let t59627 = F::new(4.0) * t4324 * t2 * t584;
    let t59629 = F::new(2.0) * t1534 * t584;
    let t59631 = F::new(6.0) * t17139 * t16;
    let t59637 = F::cast_from(0.38596750796862084161e3_f64) * t48763 * t14389;
    let t59650 = F::cast_from(0.32962962962962962963e-1_f64) * t47705 - F::cast_from(0.10987654320987654321e-1_f64) * t47707 + F::cast_from(0.82407407407407407408e-2_f64) * t47709 + F::cast_from(0.41203703703703703704e-2_f64) * t47711 + F::cast_from(0.68672839506172839507e-2_f64) * t47713 - F::cast_from(0.24722222222222222222e-1_f64) * t47715 - F::cast_from(0.12361111111111111111e-1_f64) * t47717 - F::cast_from(0.24722222222222222223e-1_f64) * t47724 - F::cast_from(0.16481481481481481482e-1_f64) * t47730 + F::cast_from(0.61805555555555555556e-2_f64) * t47732 + F::cast_from(0.37083333333333333333e-1_f64) * t47738 - F::cast_from(0.41203703703703703703e-2_f64) * t41656;
    (t59627, t59629, t59631, t59637, t59650)
}
