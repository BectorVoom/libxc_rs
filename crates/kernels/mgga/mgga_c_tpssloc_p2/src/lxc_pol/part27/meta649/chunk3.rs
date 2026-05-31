//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2253/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2253<F: Float>(t25: F, t265: F, t394: F, t89775: F, t89822: F, t89729: F, t12606: F, t1409: F, t1965: F, t2250: F, t23773: F, t25883: F, t3966: F, t40: F, t607: F, t6835: F, t7643: F, t88003: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t89823 = t89775 + t89822;
    let t89824 = piecewise3::<F>(t395, t89729, t89823);
    let t89836 = piecewise3::<F>(t115, t88003, t89824 * t40 / F::cast_from(2.0_f64) + t25883 * t607 + t7643 * t2250 / F::cast_from(2.0_f64) + t23773 * t1409 / F::cast_from(2.0_f64) + t6835 * t3966 + t1965 * t12606 / F::cast_from(2.0_f64));
    (t89823, t89836)
}
