//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1009/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1009<F: Float>(t78244: F, t25854: F, t77786: F, t27048: F, t77789: F, t27055: F, t77341: F, t41116: F, t77345: F, t75748: F, t75756: F, t71628: F) -> (F, F, F, F, F, F, F, F) {
    let t78245 = F::cast_from(0.5987120850931904282e-1_f64) * t78244;
    let t78246 = t25854 * t77786;
    let t78247 = F::cast_from(0.8980681276397856423e-1_f64) * t78246;
    let t78248 = t27048 * t77789;
    let t78249 = F::cast_from(0.8980681276397856423e-1_f64) * t78248;
    let t78251 = F::cast_from(0.35922725105591425692e0_f64) * t27055 * t77341;
    let t78253 = F::cast_from(0.47896966807455234256e0_f64) * t41116 * t77345;
    let t78271 = F::cast_from(0.79808624799933448875e-4_f64) * t75748;
    let t78272 = F::cast_from(0.212822999466489197e-4_f64) * t75756;
    let t78273 = F::cast_from(0.39914139006212695213e-1_f64) * t71628;
    (t78245, t78247, t78249, t78251, t78253, t78271, t78272, t78273)
}
