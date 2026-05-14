//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1045/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1045<F: Float>(t2512: F, t39378: F, t9489: F, t1294: F, t2509: F, t39389: F, t763: F, t9697: F, t3684: F, t2371: F, t2393: F, t2528: F, t677: F, t9722: F, t9919: F, t2535: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39488 = t9489 * t39378 * t2512;
    let t39490 = 0.6233709278045326953e3 * t1294 * t39488;
    let t39494 = t2509 * t39389 * t2512;
    let t39496 = 0.51947577317044391277e2 * t1294 * t39494;
    let t39497 = t9697 * t763;
    let t39499 = 0.67471172535210825684e-1 * t3684 * t39497;
    let t39500 = t2393 * t2371;
    let t39502 = 0.86748650402413918736e-1 * t3684 * t39500;
    let t39503 = t2393 * t2528;
    let t39505 = 0.12842595503380418954e1 * t3684 * t39503;
    let t39506 = t677 * t9722;
    let t39508 = 0.38527786510141256862e1 * t3684 * t39506;
    let t39516 = t677 * t9919;
    let t39518 = 0.1301229756036208781e0 * t3684 * t39516;
    let t39519 = t2393 * t2535;
    (t39488, t39490, t39494, t39496, t39497, t39499, t39500, t39502, t39503, t39505, t39506, t39508, t39516, t39518, t39519)
}
