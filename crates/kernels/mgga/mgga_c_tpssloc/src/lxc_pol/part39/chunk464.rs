//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 464/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk464<F: Float>(t1547: F, t894: F, t901: F, t1539: F, t908: F, t136: F, t1541: F, t899: F, t907: F, t913: F) -> (F, F, F, F, F, F) {
    let t1548 = t894 * t1547;
    let t1551 = t901 * t1547;
    let t1553 = t908 * t1539;
    let t1554 = t136 * t1553;
    let t1556 = F::new(0.1898925e1) * t1548 - t899 - F::new(0.29896666666666666667e0) * t1541 + F::new(0.3071625e0) * t1551 - t907 - F::new(0.82156666666666666667e-1) * t1554;
    let t1557 = t1556 * t913;
    (t1548, t1551, t1553, t1554, t1556, t1557)
}
