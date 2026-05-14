//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 788/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk788<F: Float>(t23270: F, t33447: F, t22986: F, t2053: F, t2718: F, t7537: F, t1527: F, t31332: F, t1888: F, t1528: F, t30748: F, t31407: F, t31423: F, t31426: F, t32877: F, t33443: F, t6627: F, t7087: F, t7517: F, t7830: F, t855: F) -> (F, F, F, F, F) {
    let t33448 = t23270 * t33447;
    let t33449 = t22986 * t33448;
    let t33452 = t2718 * t2053 * t7537;
    let t33457 = t31332 * t1527;
    let t33458 = t23270 * t33457;
    let t33459 = t1888 * t33458;
    let t33463 = 2.0 * t855 * t33443 + t31407 - t31423 * t1528 + 0.16449340668482264365e-1 * t33449 + 2.0 * t855 * t33452 + 2.0 * t6627 * t7830 + 0.16449340668482264365e-1 * t33459 - t32877 + 2.0 * t7087 * t7517 + t30748 + t31426;
    (t33448, t33452, t33457, t33458, t33463)
}
