//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1005/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1005<F: Float>(t25: F, t265: F, t394: F, t2020: F, t33746: F, t7688: F, t8690: F, t33043: F, t1409: F, t32907: F, t40: F, t8678: F, t7754: F, t1873: F, t27921: F, t24972: F, t7769: F, t7423: F, t7467: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t33747 = t33746 * t2020;
    let t33748 = t8690 * t7688;
    let t33750 = piecewise3(t395, 0.0, t33043);
    let t33755 = piecewise3(t115, t32907, t8678 * t1409 / 2.0 + t33750 * t40 / 2.0);
    let t33758 = t8690 * t7754;
    let t33774 = t27921 * t1873;
    let t33776 = t24972 * t7769;
    let t33778 = t7423 * t7467;
    (t33747, t33748, t33750, t33755, t33758, t33774, t33776, t33778)
}
