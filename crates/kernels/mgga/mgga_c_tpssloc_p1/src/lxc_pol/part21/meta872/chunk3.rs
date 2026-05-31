//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3215/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3215<F: Float>(t28: F, t265: F, t504: F, t59618: F, t64473: F, t64510: F, t64534: F, t64545: F, t66885: F, t66886: F, t66891: F, t66901: F, t1081: F, t1260: F, t12606: F, t13493: F, t1409: F, t15844: F, t1649: F, t16558: F, t17133: F, t1768: F, t18196: F, t19276: F, t2250: F, t2756: F, t3231: F, t3644: F, t3966: F, t47676: F, t506: F, t5099: F, t52: F, t5398: F, t55677: F, t5669: F, t59627: F, t59629: F, t59631: F, t5966: F, t607: F, t6279: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t66905 = piecewise3::<F>(t505, t64473 + t64510 + t64534 + t64545 + t66885 + t66886 + t66891 + t66901, t59618);
    let t66921 = piecewise3::<F>(t401, t59618 * t28 / F::cast_from(2.0_f64) + t17133 * t1081 + t5669 * t3231 / F::cast_from(2.0_f64) + t13493 * t1649 - t59627 - t59629 + t59631 + t2756 * t5966 / F::cast_from(2.0_f64) + t873 * t18196 - t47676, t66905 * t52 / F::cast_from(2.0_f64) - t19276 * t607 - t6279 * t2250 / F::cast_from(2.0_f64) - t15844 * t1409 - F::cast_from(2.0_f64) * t5099 * t3966 - t1768 * t12606 - t3644 * t5398 / F::cast_from(2.0_f64) - t1260 * t16558 - t506 * t55677 / F::cast_from(2.0_f64));
    t66921
}
