//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1091/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1091<F: Float>(t1562: F, t9639: F, t10267: F, t1668: F, t1707: F, t2211: F, t2228: F, t30344: F, t43190: F, t43204: F, t43207: F, t43211: F, t43241: F, t43817: F, t45890: F, t45896: F, t45901: F, t45905: F, t45909: F, t45911: F, t45914: F, t4601: F, t530: F, t739: F, t903: F, t9343: F) -> F {
    let t48700 = t1562 * t9639;
    let t48706 = -t43190 + F::cast_from(0.35922725105591425692e0_f64) * t4601 * t10267 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t2228 * t1707 - F::cast_from(0.5107751987195740728e-4_f64) * t45890 - F::cast_from(0.638468998399467591e-4_f64) * t45896 - F::cast_from(0.10215503974391481456e-3_f64) * t45901 + F::cast_from(0.30646511923174444368e-3_f64) * t45905 - F::cast_from(0.5107751987195740728e-3_f64) * t45909 + t43204 - t43207 + t43211 + F::cast_from(0.212822999466489197e-4_f64) * t45911 - F::new(0.4726e1) * t1668 * t9343 - F::new(0.4726e1) * t530 * t43817 - F::new(0.4726e1) * t48700 - t43241 + F::cast_from(0.5987120850931904282e-1_f64) * t45914 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t2211 * t30344;
    t48706
}
