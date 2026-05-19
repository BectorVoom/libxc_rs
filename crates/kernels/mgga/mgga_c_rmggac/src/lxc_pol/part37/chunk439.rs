//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 439/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk439<F: Float>(t321: F, t8946: F, t333: F, t118: F, t4669: F, t5155: F, t5266: F, t7793: F, t7795: F, t7815: F, t7819: F, t7821: F, t8801: F, t8804: F, t8926: F, t8933: F, t8937: F, t8940: F, t8941: F, t8944: F) -> (F, F, F) {
    let t8947 = t8946 * t321;
    let t8950 = t8946 * t333;
    let t8955 = F::cast_from(0.34093327067806677161e-2_f64) * t8926 + F::cast_from(0.39914139006212695213e-1_f64) * t7793 + F::cast_from(0.11974241701863808564e0_f64) * t7795 - F::cast_from(0.79828278012425390426e-1_f64) * t7815 + t7819 - t7821 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t8804 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t8933 + F::cast_from(0.11974241701863808564e0_f64) * t5266 * t8937 + F::cast_from(0.11974241701863808564e0_f64) * t8940 * t8941 - F::cast_from(0.14967802127329760705e-1_f64) * t8944 - F::cast_from(0.17961362552795712846e0_f64) * t4669 * t8947 + F::cast_from(0.23948483403727617128e0_f64) * t5155 * t8950 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t8801;
    (t8947, t8950, t8955)
}
