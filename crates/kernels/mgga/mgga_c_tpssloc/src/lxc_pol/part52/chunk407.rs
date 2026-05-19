//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 407/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk407<F: Float>(t1164: F, t1703: F, t1420: F, t338: F, t1178: F, t1409: F, t1177: F, t1111: F, t1668: F) -> (F, F, F, F, F) {
    let t1705 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t1703;
    let t1706 = t1420 * t338;
    let t1709 = t1178 * t1409;
    let t1710 = t1177 * t1709;
    let t1714 = t1111 / F::new(6.0) - t1668 / F::new(6.0);
    (t1705, t1706, t1709, t1710, t1714)
}
