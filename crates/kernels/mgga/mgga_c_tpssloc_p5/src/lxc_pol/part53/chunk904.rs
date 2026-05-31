//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 904/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk904<F: Float>(t33151: F, t7676: F, t8326: F, t1799: F, t3701: F, t1458: F, t576: F, t5371: F, t3941: F, t2035: F) -> (F, F, F, F, F, F, F, F) {
    let t33152 = F::cast_from(2.0_f64) * t33151;
    let t33153 = t7676 * t8326;
    let t33154 = F::cast_from(2.0_f64) * t33153;
    let t33159 = t3701 * t1799;
    let t33185 = t576 * t1458;
    let t33191 = t5371 * t8326;
    let t33192 = F::cast_from(0.135e2_f64) * t33191;
    let t33193 = t8326 * t1458;
    let t33194 = t3941 * t33193;
    let t33195 = F::cast_from(27.0_f64) * t33194;
    let t33234 = t2035 * t1458;
    (t33152, t33154, t33159, t33185, t33192, t33193, t33195, t33234)
}
