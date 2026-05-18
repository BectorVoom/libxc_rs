//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 286/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk286<F: Float>(t118: F, t2228: F, t2055: F, t2058: F, t2062: F, t2071: F, t2076: F, t2082: F, t2200: F, t2204: F, t2206: F, t2209: F, t2213: F) -> (F, F) {
    let t2229 = t118 * t2228;
    let t2231 = F::new(0.5987120850931904282e-1) * t2055 - F::new(0.8980681276397856423e-1) * t2058 - F::new(0.2993560425465952141e-1) * t2062 - t2200 - F::new(0.20455996240684006298e-1) * t2071 + F::new(0.2727466165424534173e-1) * t2076 + F::new(0.68186654135613354325e-2) * t2082 + t2204 + F::new(0.59871208509319042821e-1) * t2206 - F::new(0.59871208509319042821e-1) * t2209 - F::new(0.39914139006212695214e-1) * t2213 + F::new(0.19957069503106347607e-1) * t2229;
    (t2229, t2231)
}
