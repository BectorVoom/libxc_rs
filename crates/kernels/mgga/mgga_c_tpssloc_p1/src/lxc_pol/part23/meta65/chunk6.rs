//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 397/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk397<F: Float>(t1129: F, t1148: F, t1659: F, t1673: F, t1675: F, t1683: F, t1688: F, t1695: F, t300: F, t436: F, t1147: F, t1156: F, t1694: F) -> (F, F, F) {
    let t1699 = t300 * (-F::new(0.310907e-1) * t1675 * t436 + F::new(1.0) * t1129 * t1683 + t1659 - t1673 - F::cast_from(0.19751673498613801407e-1_f64) * t1688 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t1695);
    let t1701 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t1688;
    let t1703 = t1147 * t1694 * t1156;
    (t1699, t1701, t1703)
}
