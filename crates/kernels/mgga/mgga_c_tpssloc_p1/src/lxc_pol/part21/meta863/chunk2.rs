//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3143/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3143<F: Float>(t1174: F, t15281: F, t18554: F, t11570: F, t17635: F, t11569: F, t1177: F, t1178: F, t15390: F, t18321: F, t3443: F, t3447: F, t3457: F, t3461: F, t3475: F, t460: F, t4919: F, t4934: F, t52066: F, t52100: F, t52224: F, t52228: F, t52240: F, t52250: F, t55677: F, t6138: F) -> F {
    let t65041 = t1174 * t15281 * t18554;
    let t65056 = t11570 * t17635;
    let t65073 = -F::cast_from(0.55555555555555555554e-3_f64) * t65041 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t1177 * t1178 * t55677 + F::cast_from(0.33333333333333333333e-2_f64) * t3447 * t4919 * t52224 - F::cast_from(0.44444444444444444444e-2_f64) * t3447 * t15390 * t52228 + F::cast_from(0.17283950617283950617e-2_f64) * t3447 * t52100 * t52066 - F::cast_from(0.74074074074074074072e-3_f64) * t3447 * t11569 * t65056 + F::cast_from(0.98765432098765432094e-3_f64) * t52240 - F::cast_from(0.11111111111111111111e-2_f64) * t52250 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4934 * t6138 * t3475 * t460 - F::cast_from(0.27160493827160493827e-2_f64) * t18321 * t3461 - F::cast_from(0.54320987654320987654e-2_f64) * t18321 * t3457 + F::cast_from(0.36213991769547325103e-2_f64) * t18321 * t3443;
    t65073
}
