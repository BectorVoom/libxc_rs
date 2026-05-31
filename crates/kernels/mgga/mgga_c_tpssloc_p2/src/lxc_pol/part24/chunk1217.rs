//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1217/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1217<F: Float>(t25: F, t1965: F, t2250: F, t23309: F, t23773: F, t40: F, t607: F, t6835: F, t2379: F, t28: F, t2752: F, t13487: F, t1081: F, t776: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t23780 = piecewise3::<F>(t115, t23309, t23773 * t40 / F::cast_from(2.0_f64) + t6835 * t607 + t1965 * t2250 / F::cast_from(2.0_f64));
    let t23781 = t28 * t2379;
    let t23788 = t2752 * t28;
    let t23789 = t23788 * t13487;
    let t23792 = t1081 * t776;
    (t23780, t23781, t23788, t23789, t23792)
}
