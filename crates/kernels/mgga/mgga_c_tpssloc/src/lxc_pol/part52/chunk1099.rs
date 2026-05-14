//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1099/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1099<F: Float>(t28: F, t265: F, t504: F, t7540: F, t1649: F, t1877: F, t2522: F, t30757: F, t30770: F, t32886: F, t6670: F, t7649: F, t7656: F, t8366: F, t8370: F, t33043: F, t1409: F, t52: F, t8435: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t33065 = t28 * t7540;
    let t33073 = 3.0 / 2.0 * t2522 * t8366 * t7649 + t1877 * t32886 * t28 / 2.0 - t1877 * t30757 * t7656 / 2.0 + t1877 * t8366 * t1649 / 2.0 - 3.0 / 2.0 * t2522 * t8370 * t7649 - t1877 * t6670 * t33065 + t1877 * t30770 * t7656 - t1877 * t8370 * t1649 / 2.0;
    let t33074 = piecewise3(t505, 0.0, t33043);
    let t33079 = piecewise3(t401, t33073, -t8435 * t1409 / 2.0 + t33074 * t52 / 2.0);
    (t33065, t33074, t33079)
}
