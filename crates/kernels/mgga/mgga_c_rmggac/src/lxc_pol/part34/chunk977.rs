//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 977/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk977<F: Float>(t14451: F, t5144: F, t30204: F, t74791: F, t15605: F, t290: F, t74793: F, t74795: F, t74797: F, t34975: F, t34976: F, t699: F, t8455: F) -> (F, F, F, F, F, F, F, F) {
    let t77204 = t14451 * t5144;
    let t77205 = t30204 * t77204;
    let t77206 = F::cast_from(0.5987120850931904282e-1_f64) * t77205;
    let t77208 = F::cast_from(0.21814357445315142692e-4_f64) * t74791;
    let t77209 = t290 * t15605;
    let t77212 = F::cast_from(0.68186654135613354325e-2_f64) * t74793;
    let t77213 = F::cast_from(0.68186654135613354325e-2_f64) * t74795;
    let t77214 = F::cast_from(0.12263514265030957031e-4_f64) * t74797;
    let t77217 = t34975 * t34976 * t699 * t8455;
    (t77204, t77206, t77208, t77209, t77212, t77213, t77214, t77217)
}
