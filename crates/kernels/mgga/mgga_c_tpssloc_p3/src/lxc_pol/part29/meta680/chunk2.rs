//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2288/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2288<F: Float>(t24574: F, t27427: F, t5052: F, t7284: F, t14980: F, t15803: F, t1761: F, t2155: F, t24868: F, t27382: F, t27742: F, t3477: F, t3593: F, t4945: F, t5055: F, t51928: F, t7283: F, t7287: F, t7351: F, t7356: F, t7392: F, t86400: F, t86409: F, t86424: F) -> F {
    let t94676 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27427;
    let t94680 = t7284 * t5052;
    let t94698 = -t94676 + F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t3477 * t27382 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t94680 * t7287 - t5055 * t24868 + F::cast_from(0.12184696791468343974e-2_f64) * t86409 - F::cast_from(2.0_f64) * t14980 * t7392 - t86400 * t1761 - t4945 * t24868 + F::cast_from(2.0_f64) * t7351 * t15803 - F::cast_from(2.0_f64) * t3593 * t27742 - F::cast_from(0.27415567780803773942e-2_f64) * t86424 + F::cast_from(4.0_f64) * t14980 * t7356 - t51928 * t2155;
    t94698
}
