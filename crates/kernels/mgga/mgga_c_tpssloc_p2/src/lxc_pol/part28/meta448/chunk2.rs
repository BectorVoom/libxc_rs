//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1639/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1639<F: Float>(t1860: F, t23998: F, t2031: F, t22489: F, t2032: F, t22493: F, t22519: F, t22527: F, t22531: F, t22534: F, t22537: F, t22546: F, t22549: F, t23963: F, t23968: F, t23970: F, t23973: F, t23975: F, t23978: F, t23995: F, t6486: F, t6492: F, t6495: F, t7026: F, t7035: F) -> (F, F, F) {
    let t23999 = t1860 * t23998;
    let t24001 = t2031 * t22489;
    let t24006 = F::new(10.0) * t23963 * t22546 + F::new(80.0) / F::new(9.0) * t23968 + F::new(20.0) / F::new(3.0) * t22549 * t23970 + F::new(32.0) / F::new(9.0) * t23973 - F::new(10.0) / F::new(3.0) * t23975 * t6492 - F::new(16.0) / F::new(9.0) * t23978 - F::new(4.0) / F::new(3.0) * t22519 * t2032 - F::new(10.0) / F::new(3.0) * t7026 * t22527 - F::new(5.0) / F::new(3.0) * t7026 * t22531 - F::new(2.0) / F::new(3.0) * t22534 * t2032 - F::new(2.0) / F::new(3.0) * t22537 * t2032 - F::new(4.0) / F::new(3.0) * t6495 * t7035 + t23995 + F::new(2.0) / F::new(3.0) * t6486 * t7035 - F::new(16.0) / F::new(9.0) * t23999 + t1860 * t24001 / F::new(3.0) + t22493 * t2032 / F::new(3.0);
    (t23999, t24001, t24006)
}
