//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1014/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1014<F: Float>(t1238: F, t14980: F, t1761: F, t2155: F, t24589: F, t24880: F, t27406: F, t27422: F, t27424: F, t27427: F, t27434: F, t27438: F, t27441: F, t27446: F, t27742: F, t27747: F, t27752: F, t27755: F, t3487: F, t498: F, t7283: F, t7288: F, t8061: F) -> F {
    let t27757 = t27422 * t498 + t27424 * t498 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t27427 + F::cast_from(0.73108180748810063843e-2_f64) * t27406 * t7288 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t27434 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t27438 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t27441 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t27446 - t14980 * t2155 - t1238 * t27742 + F::new(2.0) * t3487 * t8061 + F::new(2.0) * t1238 * t27747 - t24880 * t1761 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27752 - F::cast_from(0.27415567780803773942e-2_f64) * t27755;
    t27757
}
