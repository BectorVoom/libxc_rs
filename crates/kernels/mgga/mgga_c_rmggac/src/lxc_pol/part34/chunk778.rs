//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 778/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk778<F: Float>(t3851: F, t75886: F, t75216: F, t793: F, t41400: F, t649: F, t8950: F, t40932: F, t8979: F, t5148: F, t76077: F, t69453: F, t5259: F, t74959: F, t4669: F, t74963: F) -> (F, F, F, F, F, F, F, F) {
    let t76333 = t3851 * t75886;
    let t76337 = t793 * t75216;
    let t76340 = t41400 * t649 * t8950;
    let t76343 = t40932 * t649 * t8979;
    let t76355 = 0.5987120850931904282e-1 * t5148 * t76077;
    let t76356 = 0.79828278012425390427e-1 * t69453;
    let t76358 = 0.5987120850931904282e-1 * t5259 * t74959;
    let t76360 = 0.8980681276397856423e-1 * t4669 * t74963;
    (t76333, t76337, t76340, t76343, t76355, t76356, t76358, t76360)
}
