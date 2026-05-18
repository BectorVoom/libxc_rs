//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1002/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1002<F: Float>(t2471: F, t265: F, t305: F, t76373: F, t76375: F, t76377: F, t76379: F, t76381: F, t69213: F, t69234: F, t69241: F, t69250: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t78112 = t2471 * t265;
    let t78113 = t305 * t78112;
    let t78114 = F::new(0.39914139006212695213e-1) * t78113;
    let t78115 = F::new(0.20455996240684006298e-1) * t76373;
    let t78116 = F::new(0.20455996240684006298e-1) * t76375;
    let t78117 = F::new(0.2727466165424534173e-1) * t76377;
    let t78119 = F::new(0.2727466165424534173e-1) * t76379;
    let t78120 = F::new(0.54549323308490683461e-1) * t76381;
    let t78122 = F::new(0.77145928569998943516e-3) * t69213;
    let t78123 = F::new(0.16566831523319392755e-1) * t69234;
    let t78124 = F::new(0.27611385872198987926e-1) * t69241;
    let t78125 = F::new(0.72732431077987577944e-1) * t69250;
    (t78112, t78114, t78115, t78116, t78117, t78119, t78120, t78122, t78123, t78124, t78125)
}
