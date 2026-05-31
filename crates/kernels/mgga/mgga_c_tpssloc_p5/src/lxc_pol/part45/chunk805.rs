//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 805/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk805<F: Float>(t28: F, t265: F, t504: F, t23772: F, t1972: F, t2250: F, t23820: F, t52: F, t607: F, t6856: F, t23780: F, t1873: F, t3652: F, t652: F, t6876: F, t7000: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t23821 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t23772);
    let t23828 = piecewise3::<F>(t401, t23820, t23821 * t52 / F::cast_from(2.0_f64) - t6856 * t607 - t1972 * t2250 / F::cast_from(2.0_f64));
    let t23829 = t23780 + t23828;
    let t23831 = t3652 * t1873;
    let t23833 = F::cast_from(2.0_f64) * t652 * t23831;
    let t23835 = F::cast_from(2.0_f64) * t6876 * t7000;
    (t23829, t23831, t23833, t23835)
}
