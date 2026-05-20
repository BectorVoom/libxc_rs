//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2282/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2282<F: Float>(t7303: F, t94490: F, t7291: F, t11605: F, t1186: F, t1251: F, t1761: F, t2155: F, t24589: F, t24601: F, t24602: F, t27761: F, t27766: F, t27784: F, t3487: F, t3966: F, t5059: F, t51937: F, t7283: F, t7391: F, t8002: F, t85711: F, t85717: F, t85724: F, t85733: F, t94475: F, t94476: F) -> F {
    let t94492 = F::cast_from(0.14621636149762012769e-1_f64) * t94490 * t7303;
    let t94494 = F::cast_from(0.14621636149762012769e-1_f64) * t94490 * t7291;
    let t94498 = F::new(4.0) * t3487 * t27761 - F::cast_from(0.27415567780803773942e-2_f64) * t85711 - t51937 * t2155 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t27766 - t94475 + F::cast_from(0.18277045187202515961e-2_f64) * t94476 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t24602 * t3966 * t1251 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t85724 * t8002 - F::new(12.0) * t27784 * t11605 * t7391 * t5059 + t94492 + t94494 + F::cast_from(0.54831135561607547884e-2_f64) * t85733 - F::new(2.0) * t85717 * t1761;
    t94498
}
