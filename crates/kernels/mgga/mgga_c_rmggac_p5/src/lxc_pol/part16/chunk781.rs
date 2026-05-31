//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 781/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk781<F: Float>(t7767: F, t8201: F, t7901: F, t34687: F, t34704: F, t34706: F, t34710: F, t34752: F, t34772: F, t34784: F, t34787: F, t34793: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37179 = F::cast_from(0.18292589874945016987e-2_f64) * t7767;
    let t37183 = F::cast_from(3.0_f64) * t8201;
    let t37186 = F::cast_from(0.87811105813667929468e0_f64) * t7901;
    let t37200 = F::cast_from(0.205201155180140685e-5_f64) * t34687;
    let t37201 = F::cast_from(0.18292589874945016987e-2_f64) * t34704;
    let t37202 = F::cast_from(0.91462949374725084936e-3_f64) * t34706;
    let t37203 = F::cast_from(0.13010691197123848592e-3_f64) * t34710;
    let t37214 = F::cast_from(0.205201155180140685e-5_f64) * t34752;
    let t37218 = F::cast_from(0.30487649791575028312e-3_f64) * t34772;
    let t37221 = F::cast_from(0.91462949374725084936e-3_f64) * t34784;
    let t37222 = F::cast_from(0.13010691197123848592e-3_f64) * t34787;
    let t37223 = F::cast_from(0.18292589874945016987e-2_f64) * t34793;
    (t37179, t37183, t37186, t37200, t37201, t37202, t37203, t37214, t37218, t37221, t37222, t37223)
}
