//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 937/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk937<F: Float>(t74327: F, t74330: F, t74333: F, t74337: F, t74339: F, t74345: F, t74354: F, t74356: F, t74368: F, t74371: F, t74374: F, t3351: F, t3352: F, t875: F, t9577: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t76969 = F::cast_from(0.3192344991997337955e-4_f64) * t74327;
    let t76970 = F::cast_from(0.47885174879960069325e-4_f64) * t74330;
    let t76972 = F::cast_from(0.15961724959986689775e-4_f64) * t74333;
    let t76973 = F::cast_from(0.2553875993597870364e-4_f64) * t74337;
    let t76974 = F::cast_from(0.1702583995731913576e-4_f64) * t74339;
    let t76975 = F::cast_from(0.1702583995731913576e-4_f64) * t74345;
    let t76976 = F::cast_from(0.1702583995731913576e-4_f64) * t74354;
    let t76977 = F::cast_from(0.85129199786595678799e-5_f64) * t74356;
    let t76978 = F::cast_from(0.85129199786595678799e-5_f64) * t74368;
    let t76979 = F::cast_from(0.15961724959986689775e-4_f64) * t74371;
    let t76980 = F::cast_from(0.1276937996798935182e-4_f64) * t74374;
    let t76985 = t3351 * t3352 * t875 * t9577;
    (t76969, t76970, t76972, t76973, t76974, t76975, t76976, t76977, t76978, t76979, t76980, t76985)
}
