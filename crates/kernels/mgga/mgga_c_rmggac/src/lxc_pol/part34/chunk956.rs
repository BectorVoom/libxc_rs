//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 956/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk956<F: Float>(t74267: F, t74269: F, t74272: F, t74275: F, t70905: F, t74238: F, t74243: F, t76935: F, t76937: F, t76939: F, t76940: F, t76941: F, t76942: F, t76943: F, t76946: F, t76947: F, t76948: F) -> F {
    let t76949 = F::new(0.16263363996404810741e-4) * t74267;
    let t76950 = F::new(0.38430329123504567781e-4) * t74269;
    let t76951 = F::new(0.16263363996404810741e-4) * t74272;
    let t76952 = F::new(0.72042316457491791901e-3) * t74275;
    let t76953 = t76935 - F::new(0.52557918278704101564e-6) * t74238 + t76937 + F::new(0.76860658247009135557e-5) * t74243 - t76939 - t76940 + t76941 + t76942 - t70905 - t76943 - t76946 - t76947 - t76948 + t76949 + t76950 + t76951 - t76952;
    t76953
}
