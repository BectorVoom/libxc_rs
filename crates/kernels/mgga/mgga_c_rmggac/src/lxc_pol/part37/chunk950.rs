//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 950/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk950<F: Float>(t71112: F, t14451: F, t5144: F, t30204: F, t74791: F, t74793: F, t74795: F, t74797: F, t34975: F, t34976: F, t699: F, t8455: F) -> (F, F, F, F, F, F, F, F) {
    let t77197 = F::new(0.29795219925308487579e-4) * t71112;
    let t77204 = t14451 * t5144;
    let t77205 = t30204 * t77204;
    let t77206 = F::new(0.5987120850931904282e-1) * t77205;
    let t77208 = F::new(0.21814357445315142692e-4) * t74791;
    let t77212 = F::new(0.68186654135613354325e-2) * t74793;
    let t77213 = F::new(0.68186654135613354325e-2) * t74795;
    let t77214 = F::new(0.12263514265030957031e-4) * t74797;
    let t77217 = t34975 * t34976 * t699 * t8455;
    (t77197, t77204, t77206, t77208, t77212, t77213, t77214, t77217)
}
