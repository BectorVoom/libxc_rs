//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3020/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3020<F: Float>(t25: F, t265: F, t394: F, t59618: F, t60840: F, t60878: F, t60904: F, t60909: F, t60924: F, t60939: F, t60962: F, t63241: F, t1074: F, t12606: F, t13493: F, t1408: F, t1409: F, t14675: F, t1642: F, t16557: F, t16558: F, t17133: F, t18176: F, t2249: F, t2250: F, t2756: F, t3220: F, t396: F, t3966: F, t40: F, t4705: F, t47676: F, t5397: F, t5398: F, t55677: F, t5669: F, t5955: F, t59627: F, t59629: F, t59631: F, t606: F, t607: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t63245 = piecewise3::<F>(t395, t60840 + t60878 + t60904 + t60909 + t60924 + t60939 + t60962 + t63241, t59618);
    let t63261 = piecewise3::<F>(t115, t59618 * t25 / F::new(2.0) + t17133 * t606 + t5669 * t2249 / F::new(2.0) + t13493 * t1408 + t59627 + t59629 - t59631 + t2756 * t5397 / F::new(2.0) + t873 * t16557 + t47676, t63245 * t40 / F::new(2.0) + t18176 * t607 + t5955 * t2250 / F::new(2.0) + t14675 * t1409 + F::new(2.0) * t4705 * t3966 + t1642 * t12606 + t3220 * t5398 / F::new(2.0) + t1074 * t16558 + t396 * t55677 / F::new(2.0));
    t63261
}
