//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1050/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1050<F: Float>(t76363: F, t76365: F, t2471: F, t265: F, t305: F, t76373: F, t76375: F, t76377: F, t76356: F, t76358: F, t76360: F, t76362: F, t76368: F, t76370: F, t76372: F) -> (F, F) {
    let t78110 = F::new(0.10909864661698136691e0) * t76363;
    let t78111 = F::new(0.21819729323396273382e0) * t76365;
    let t78112 = t2471 * t265;
    let t78113 = t305 * t78112;
    let t78114 = F::new(0.39914139006212695213e-1) * t78113;
    let t78115 = F::new(0.20455996240684006298e-1) * t76373;
    let t78116 = F::new(0.20455996240684006298e-1) * t76375;
    let t78117 = F::new(0.2727466165424534173e-1) * t76377;
    let t78118 = t76356 - t76358 + t76360 + t76362 - t78110 + t78111 - t76368 + t76370 - t76372 + t78114 + t78115 + t78116 - t78117;
    (t78112, t78118)
}
