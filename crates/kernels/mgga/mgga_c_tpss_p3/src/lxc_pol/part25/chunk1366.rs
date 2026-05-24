//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1366/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1366<F: Float>(t198: F, t205: F, t6353: F, t1692: F, t1989: F, t6354: F, t18728: F, t69868: F, t18807: F, t19672: F, t19685: F, t20417: F, t21263: F, t21266: F, t21353: F, t21359: F, t2439: F, t4578: F, t5849: F, t5853: F, t62610: F, t62829: F, t69789: F, t69811: F, t69828: F, t69842: F, t70227: F, t70261: F) -> (F, F, F, F) {
    let t72279 = t198 * t205 * t6353;
    let t72298 = F::new(2.0) * t1692 * t6354 * t1989;
    let t72310 = F::new(6.0) * t18728 * t69868;
    let t72317 = F::new(6.0) * t72279 * t19672 + F::new(3.0) * t2439 * t5849 * t21266 - t1692 * t5853 * t70261 / F::new(2.0) - F::new(3.0) * t62610 * t21263 + F::new(3.0) * t2439 * t6354 * t19685 - t1692 * t5853 * t70227 / F::new(2.0) + t72298 + t1692 * t62829 * t21353 + t1692 * t5849 * t4578 / F::new(2.0) - F::new(3.0) * t18728 * t69789 - t1692 * t18807 * t21359 / F::new(2.0) + t72310 + F::new(3.0) * t20417 * t69842 - F::new(3.0) * t18728 * t69828 - F::new(3.0) * t18728 * t69811;
    (t72279, t72298, t72310, t72317)
}
