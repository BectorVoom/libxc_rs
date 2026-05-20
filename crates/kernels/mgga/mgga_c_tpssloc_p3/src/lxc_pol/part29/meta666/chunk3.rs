//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2222/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2222<F: Float>(t225: F, t3787: F, t562: F, t16313: F, t91004: F, t22751: F, t26385: F, t16068: F, t1992: F, t6976: F, t81149: F, t16060: F, t26403: F, t3856: F, t5250: F, t5334: F, t5344: F, t6988: F, t81115: F, t81125: F, t81127: F, t81140: F, t81147: F, t81154: F, t90942: F, t90988: F, t90993: F, t91000: F, t91002: F) -> (F, F) {
    let t91005 = t225 * t3787;
    let t91006 = t91005 * t562;
    let t91008 = t91004 * t91006 * t16313;
    let t91010 = t22751 * t26385;
    let t91011 = F::cast_from(0.76763589786250567036e-1_f64) * t91010;
    let t91014 = t1992 * t6976 * t16068;
    let t91018 = F::cast_from(0.16449340668482264365e-1_f64) * t81149;
    let t91019 = -t90988 + F::new(4.0) * t5334 * t90942 * t5250 - F::cast_from(0.82246703342411321824e-2_f64) * t90993 + F::cast_from(0.41123351671205660912e-2_f64) * t81115 - t5344 * t26403 * t3856 + F::cast_from(0.41123351671205660912e-2_f64) * t81125 + F::cast_from(0.38381794893125283518e-1_f64) * t81127 - F::cast_from(0.63969658155208805863e-1_f64) * t91000 - F::cast_from(0.2302907693587517011e0_f64) * t91002 - F::cast_from(0.6579736267392905746e-1_f64) * t91008 + t91011 - F::cast_from(0.24674011002723396547e-1_f64) * t81140 - t81147 - F::cast_from(0.16449340668482264365e-1_f64) * t91014 - F::new(2.0) * t16060 * t6988 - t91018 + t81154;
    (t91005, t91019)
}
