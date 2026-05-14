//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1351/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1351<F: Float>(t2358: F, t2248: F, t35761: F, t2350: F, t2354: F, t39108: F, t35577: F, t2342: F, t100: F, t103: F, t2336: F, t2341: F, t2343: F, t2346: F, t2349: F, t657: F, t660: F, t92: F, t9276: F, t9374: F, t9384: F, t9386: F, t9389: F, t9390: F, t9393: F, t9394: F, t9398: F, t9403: F, t9407: F, t95: F, t96: F, tau0: F) -> (F, F) {
    let t45444 = t2358 * t2358;
    let t45453 = t2248 * t2248;
    let t45460 = 1.0 / t35761;
    let t45461 = t2350 * t2350;
    let t45469 = t2354 * t2354;
    let t45482 = 12.0 * t39108;
    let t45496 = 1.0 / t35577;
    let t45497 = t2342 * t2342;
    let t45505 = 6160.0 / 81.0 * tau0 * t9276 * t96 + 10.0 / 3.0 * t92 * t2341 * t45453 + 40.0 / 9.0 * t92 * t9389 * t9393 + 40.0 / 81.0 * t100 * t45460 * t45461 - 20.0 / 9.0 * t100 * t9398 * t2350 * t2354 + 10.0 / 3.0 * t100 * t2349 * t45469 + 40.0 / 9.0 * t100 * t9403 * t9407 - 8800.0 / 81.0 * t9374 * t660 + 400.0 / 9.0 * t2336 * t2346 - 100.0 / 9.0 * t657 * t9394 + 5.0 / 3.0 * t92 * t95 * t45482 - 5.0 / 3.0 * t100 * t103 * t45482 + 800.0 / 27.0 * t2336 * t2343 + 200.0 / 81.0 * t657 * t9386 - 200.0 / 9.0 * t657 * t9390 + 40.0 / 81.0 * t92 * t45496 * t45497 - 20.0 / 9.0 * t92 * t9384 * t2342 * t2248;
    (t45444, t45505)
}
