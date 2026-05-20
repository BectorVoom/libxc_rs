//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2283/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2283<F: Float>(t2122: F, t94319: F, t8034: F, t8003: F, t85660: F, t1186: F, t11928: F, t15786: F, t24582: F, t24589: F, t24604: F, t24633: F, t27388: F, t27396: F, t27830: F, t3487: F, t3600: F, t5055: F, t7283: F, t7300: F, t7301: F, t8014: F, t8061: F, t85707: F, t85739: F, t85741: F, t85766: F) -> F {
    let t94503 = t2122 * t94319;
    let t94514 = t8034 * t2122;
    let t94525 = t85660 * t8003;
    let t94530 = -F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t24633 * t27388 - F::cast_from(0.54831135561607547884e-2_f64) * t85739 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t94503 - F::cast_from(0.18277045187202515961e-2_f64) * t85741 + F::new(4.0) * t3487 * t27396 + F::new(4.0) * t5055 * t24582 + F::new(2.0) * t11928 * t8061 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94514 * t24604 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t7300 * t7301 * t15786 + F::new(2.0) * t27830 * t3600 - F::cast_from(0.54831135561607547884e-2_f64) * t85766 + F::cast_from(0.60923483957341719871e-3_f64) * t94525 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t85707 * t8014;
    t94530
}
