//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 905/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk905<F: Float>(t25820: F, t74977: F, t14174: F, t15093: F, t14170: F, t75411: F, t4669: F, t74805: F, t15087: F, t40826: F, t5259: F, t76048: F) -> (F, F, F, F, F, F) {
    let t76199 = t25820 * t74977;
    let t76201 = t15093 * t14174;
    let t76203 = t75411 * t14170;
    let t76212 = F::new(0.8980681276397856423e-1) * t4669 * t74805;
    let t76216 = F::new(0.5987120850931904282e-1) * t40826 * t15087;
    let t76218 = F::new(0.5987120850931904282e-1) * t5259 * t76048;
    (t76199, t76201, t76203, t76212, t76216, t76218)
}
