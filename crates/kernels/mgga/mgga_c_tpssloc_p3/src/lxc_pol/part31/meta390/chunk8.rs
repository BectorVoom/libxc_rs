//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1402/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1402<F: Float>(t25: F, t265: F, t394: F, t17133: F, t18173: F, t18174: F, t1074: F, t1408: F, t1409: F, t1642: F, t16557: F, t16558: F, t17141: F, t396: F, t3966: F, t40: F, t4324: F, t4705: F, t5397: F, t5398: F, t5669: F, t5955: F, t606: F, t607: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t18176 = piecewise3::<F>(t395, t18173 + t18174, t17133);
    let t18188 = piecewise3::<F>(t115, t17133 * t25 / F::cast_from(2.0_f64) + t5669 * t606 / F::cast_from(2.0_f64) + t4324 * t1408 + t17141 + t873 * t5397 / F::cast_from(2.0_f64) + t265 * t16557 / F::cast_from(2.0_f64), t18176 * t40 / F::cast_from(2.0_f64) + t5955 * t607 / F::cast_from(2.0_f64) + t4705 * t1409 + t1642 * t3966 + t1074 * t5398 / F::cast_from(2.0_f64) + t396 * t16558 / F::cast_from(2.0_f64));
    t18188
}
