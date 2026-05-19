//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 964/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk964<F: Float>(t75024: F, t75033: F, t75037: F, t1986: F, t2464: F, t7720: F, t75051: F, t75060: F, t75077: F, t75084: F, t75088: F, t75096: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77428 = F::cast_from(0.638468998399467591e-4_f64) * t75024;
    let t77430 = F::cast_from(0.23268647941669485538e-4_f64) * t75033;
    let t77431 = F::cast_from(0.23268647941669485538e-4_f64) * t75037;
    let t77435 = t1986 * t2464;
    let t77436 = t7720 * t77435;
    let t77437 = F::cast_from(0.12769379967989351819e-4_f64) * t77436;
    let t77439 = F::cast_from(0.5255791827870410156e-5_f64) * t75051;
    let t77441 = F::cast_from(0.85129199786595678799e-5_f64) * t75060;
    let t77445 = F::cast_from(0.16263363996404810741e-4_f64) * t75077;
    let t77447 = F::cast_from(0.81300399444200075499e-3_f64) * t75084;
    let t77450 = F::cast_from(0.36366215538993788973e-1_f64) * t75088;
    let t77452 = F::cast_from(0.11634323970834742769e-4_f64) * t75096;
    (t77428, t77430, t77431, t77437, t77439, t77441, t77445, t77447, t77450, t77452)
}
