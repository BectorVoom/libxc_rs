//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 946/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk946<F: Float>(t77131: F, t74650: F, t74652: F, t74657: F, t68753: F, t74674: F, t16503: F, t2211: F, t34976: F, t8435: F, t15450: F, t34761: F) -> (F, F, F, F, F, F, F, F) {
    let t77132 = F::cast_from(0.29795219925308487578e-4_f64) * t77131;
    let t77134 = F::cast_from(0.2627895913935205078e-5_f64) * t74650;
    let t77135 = F::cast_from(0.12263514265030957031e-4_f64) * t74652;
    let t77137 = F::cast_from(0.54549323308490683456e-1_f64) * t74657;
    let t77138 = F::cast_from(0.54549323308490683456e-1_f64) * t68753;
    let t77143 = F::cast_from(0.1702583995731913576e-4_f64) * t74674;
    let t77147 = t16503 * t34976 * t2211 * t8435;
    let t77148 = F::cast_from(0.85129199786595678796e-5_f64) * t77147;
    let t77149 = t34761 * t15450;
    (t77132, t77134, t77135, t77137, t77138, t77143, t77148, t77149)
}
