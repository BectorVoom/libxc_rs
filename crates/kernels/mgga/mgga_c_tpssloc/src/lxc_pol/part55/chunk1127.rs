//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1127/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1127<F: Float>(t28: F, t1409: F, t33073: F, t34366: F, t52: F, t8909: F, t33755: F, t1458: F, t32609: F, t33148: F, t33150: F, t33152: F, t33154: F, t33711: F, t33713: F, t33715: F, t34229: F, t8446: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t34371 = piecewise3::<F>(t401, t33073, -t8909 * t1409 / F::new(2.0) + t34366 * t52 / F::new(2.0));
    let t34372 = t33755 + t34371;
    let t34381 = F::new(2.0) * t1458 * t32609 + t33148 + t33150 + t33152 + t33154 + F::new(4.0) * t33711 + F::new(4.0) * t33713 + F::new(4.0) * t33715 + t34229 + t8446;
    (t34372, t34381)
}
