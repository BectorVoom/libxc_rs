//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 800/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk800<F: Float>(t7387: F, t7492: F, t7559: F, t7562: F, t7767: F, t8201: F, t7901: F, t34687: F, t34704: F, t34706: F, t34710: F, t34752: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37108 = F::cast_from(0.91462949374725084936e-3_f64) * t7387;
    let t37134 = F::cast_from(0.18292589874945016987e-2_f64) * t7492;
    let t37147 = F::cast_from(0.26021382394247697185e-3_f64) * t7559;
    let t37148 = F::cast_from(0.20001418546446583935e0_f64) * t7562;
    let t37179 = F::cast_from(0.18292589874945016987e-2_f64) * t7767;
    let t37183 = F::cast_from(3.0_f64) * t8201;
    let t37186 = F::cast_from(0.87811105813667929468e0_f64) * t7901;
    let t37200 = F::cast_from(0.205201155180140685e-5_f64) * t34687;
    let t37201 = F::cast_from(0.18292589874945016987e-2_f64) * t34704;
    let t37202 = F::cast_from(0.91462949374725084936e-3_f64) * t34706;
    let t37203 = F::cast_from(0.13010691197123848592e-3_f64) * t34710;
    let t37214 = F::cast_from(0.205201155180140685e-5_f64) * t34752;
    (t37108, t37134, t37147, t37148, t37179, t37183, t37186, t37200, t37201, t37202, t37203, t37214)
}
