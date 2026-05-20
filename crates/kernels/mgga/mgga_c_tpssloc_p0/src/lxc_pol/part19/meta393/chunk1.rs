//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1492/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1492<F: Float>(t2342: F, t100: F, t103: F, t2248: F, t2336: F, t2341: F, t2343: F, t2346: F, t2349: F, t2350: F, t2354: F, t45453: F, t45460: F, t45461: F, t45469: F, t45482: F, t45496: F, t657: F, t660: F, t92: F, t9276: F, t9374: F, t9384: F, t9386: F, t9389: F, t9390: F, t9393: F, t9394: F, t9398: F, t9403: F, t9407: F, t95: F, t96: F, tau0: F) -> F {
    let t45497 = t2342 * t2342;
    let t45505 = F::new(6160.0) / F::new(81.0) * tau0 * t9276 * t96 + F::new(10.0) / F::new(3.0) * t92 * t2341 * t45453 + F::new(40.0) / F::new(9.0) * t92 * t9389 * t9393 + F::new(40.0) / F::new(81.0) * t100 * t45460 * t45461 - F::new(20.0) / F::new(9.0) * t100 * t9398 * t2350 * t2354 + F::new(10.0) / F::new(3.0) * t100 * t2349 * t45469 + F::new(40.0) / F::new(9.0) * t100 * t9403 * t9407 - F::new(8800.0) / F::new(81.0) * t9374 * t660 + F::new(400.0) / F::new(9.0) * t2336 * t2346 - F::new(100.0) / F::new(9.0) * t657 * t9394 + F::new(5.0) / F::new(3.0) * t92 * t95 * t45482 - F::new(5.0) / F::new(3.0) * t100 * t103 * t45482 + F::new(800.0) / F::new(27.0) * t2336 * t2343 + F::new(200.0) / F::new(81.0) * t657 * t9386 - F::new(200.0) / F::new(9.0) * t657 * t9390 + F::new(40.0) / F::new(81.0) * t92 * t45496 * t45497 - F::new(20.0) / F::new(9.0) * t92 * t9384 * t2342 * t2248;
    t45505
}
