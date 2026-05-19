//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 989/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk989<F: Float>(t5259: F, t551: F, t71949: F, t76201: F, t76203: F, t69158: F, t69162: F, t69164: F, t2447: F, t36: F, t321: F, t333: F, t4669: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77886 = t5259 * t71949 * t551;
    let t77887 = F::cast_from(0.79828278012425390427e-1_f64) * t77886;
    let t77888 = F::cast_from(0.14967802127329760705e-1_f64) * t76201;
    let t77889 = F::cast_from(0.44903406381989282115e-1_f64) * t76203;
    let t77898 = F::cast_from(0.54549323308490683461e-1_f64) * t69158;
    let t77899 = F::cast_from(0.72732431077987577948e-1_f64) * t69162;
    let t77900 = F::cast_from(0.36366215538993788974e-1_f64) * t69164;
    let t77901 = t2447 * t36;
    let t77903 = t5259 * t77901 * t321;
    let t77904 = F::cast_from(0.2993560425465952141e-1_f64) * t77903;
    let t77906 = t4669 * t77901 * t333;
    (t77887, t77888, t77889, t77898, t77899, t77900, t77901, t77904, t77906)
}
