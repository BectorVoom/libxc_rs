//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 360/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk360<F: Float>(t1539: F, t882: F, t123: F, t881: F, t291: F, t880: F, t894: F, t901: F, t908: F, t136: F, t899: F, t907: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1540 = t882 * t1539;
    let t1541 = t123 * t1540;
    let t1543 = -t881 - F::cast_from(0.17808333333333333333e-1_f64) * t1541;
    let t1545 = F::new(0.621814e-1) * t1543 * t291;
    let t1547 = -t880 / F::new(3.0) - t1541 / F::new(3.0);
    let t1548 = t894 * t1547;
    let t1551 = t901 * t1547;
    let t1553 = t908 * t1539;
    let t1554 = t136 * t1553;
    let t1556 = F::new(0.1898925e1) * t1548 - t899 - F::cast_from(0.29896666666666666667e0_f64) * t1541 + F::new(0.3071625e0) * t1551 - t907 - F::cast_from(0.82156666666666666667e-1_f64) * t1554;
    (t1540, t1541, t1543, t1545, t1547, t1548, t1551, t1553, t1554, t1556)
}
