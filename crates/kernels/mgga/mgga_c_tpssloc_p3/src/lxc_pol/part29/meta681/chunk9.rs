//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2302/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2302<F: Float>(t24574: F, t27474: F, t27492: F, t85853: F, t27498: F, t1215: F, t1244: F, t1246: F, t15239: F, t2144: F, t24833: F, t24858: F, t27520: F, t27721: F, t3624: F, t3625: F, t4733: F, t7283: F, t7362: F, t7373: F, t8073: F, t85920: F, t85988: F, t85996: F, t86000: F, t95109: F) -> F {
    let t95125 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27474;
    let t95134 = F::cast_from(0.10966227112321509577e-1_f64) * t85853 * t27492;
    let t95136 = F::cast_from(0.54831135561607547884e-2_f64) * t85853 * t27498;
    let t95150 = -F::cast_from(0.27415567780803773942e-2_f64) * t85988 - t95125 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t7362 * t24858 * t4733 - t3624 * t95109 * t3625 + F::cast_from(0.18277045187202515961e-2_f64) * t85996 + t86000 + t95134 - t95136 - F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t85920 * t8073 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t24833 * t27520 + F::cast_from(2.0_f64) * t1244 * t27721 * t1215 * t1246 + t1244 * t2144 * t15239 * t1246;
    t95150
}
