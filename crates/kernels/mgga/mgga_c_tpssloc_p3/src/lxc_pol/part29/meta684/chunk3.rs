//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2329/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2329<F: Float>(t2121: F, t3427: F, t8077: F, t27517: F, t85639: F, t24574: F, t27481: F, t11888: F, t11904: F, t15022: F, t15247: F, t24589: F, t24794: F, t24798: F, t24841: F, t24849: F, t27516: F, t27532: F, t27543: F, t3565: F, t3624: F, t5064: F, t5072: F, t7327: F, t8082: F, t8085: F, t86057: F) -> F {
    let t95726 = t2121 * t3427 * t8077;
    let t95747 = F::cast_from(0.18277045187202515961e-2_f64) * t85639 * t27517;
    let t95751 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27481;
    let t95752 = t3565 * t8085 - F::cast_from(0.18277045187202515961e-2_f64) * t95726 - t3624 * t8082 * t15022 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t7327 * t5072 * t27532 + F::new(4.0) * t11904 * t27543 + F::cast_from(0.27415567780803773942e-2_f64) * t86057 - F::new(6.0) * t11888 * t8082 * t15247 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t27516 * t24794 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t27516 * t24798 + t95747 + F::new(2.0) * t5064 * t24841 - t95751;
    t95752
}
