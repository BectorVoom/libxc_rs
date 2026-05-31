//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 993/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk993<F: Float>(t1985: F, t22662: F, t31611: F, t31560: F, t6914: F, t113950: F, t113956: F, t113961: F, t113963: F, t114140: F, t115368: F, t115372: F, t115378: F, t115417: F, t115436: F, t115480: F, t115498: F, t12033: F, t1375: F, t1378: F, t2091: F, t22904: F, t24082: F, t24139: F, t26224: F, t31653: F, t3887: F, t3912: F, t6958: F, t6962: F, t6993: F, t8627: F, t93818: F) -> F {
    let t115506 = t1985 * t31611 * t22662;
    let t115508 = t6914 * t31560;
    let t115513 = -F::cast_from(0.82246703342411321825e-2_f64) * t115368 - t31653 * t3912 + F::cast_from(0.49348022005446793095e-1_f64) * t115372 - F::cast_from(12.0_f64) * t26224 * t93818 * t6962 + F::cast_from(0.16449340668482264365e-1_f64) * t115378 + t113950 + F::cast_from(2.0_f64) * t1375 * t3887 * t2091 * t22904 - t1375 * t1378 * (t115417 + t115436 + t115480 + t115498) - F::cast_from(2.0_f64) * t24082 * t6993 - t113956 - F::cast_from(0.82246703342411321825e-2_f64) * t115506 - t113961 - t113963 - F::cast_from(0.76763589786250567036e-1_f64) * t115508 + F::cast_from(2.0_f64) * t12033 * t8627 + t114140 - t6958 * t24139;
    t115513
}
