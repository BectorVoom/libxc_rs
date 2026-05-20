//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2500/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2500<F: Float>(t25: F, t265: F, t394: F, t68418: F, t68765: F, t68897: F, t68931: F, t68999: F, t69031: F, t69462: F, t69464: F, t71055: F, t1074: F, t1408: F, t1409: F, t1534: F, t1642: F, t16557: F, t16558: F, t17133: F, t18176: F, t20216: F, t20217: F, t21076: F, t21703: F, t396: F, t3966: F, t40: F, t4324: F, t4705: F, t5397: F, t5398: F, t5955: F, t606: F, t607: F, t67059: F, t67060: F, t68427: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t71059 = piecewise3::<F>(t395, t68765 + t68897 + t68931 + t68999 + t69031 + t69462 + t69464 + t71055, t68418);
    let t71077 = piecewise3::<F>(t115, t68418 * t25 / F::new(2.0) + t21076 * t606 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t17133 * t1408 + t68427 + F::new(3.0) / F::new(2.0) * t4324 * t5397 + F::new(3.0) / F::new(2.0) * t1534 * t16557 + t873 * t20216 / F::new(2.0) + t265 * t67059 / F::new(2.0), t71059 * t40 / F::new(2.0) + t21703 * t607 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t18176 * t1409 + F::new(3.0) / F::new(2.0) * t5955 * t3966 + F::new(3.0) / F::new(2.0) * t4705 * t5398 + F::new(3.0) / F::new(2.0) * t1642 * t16558 + t1074 * t20217 / F::new(2.0) + t396 * t67060 / F::new(2.0));
    t71077
}
