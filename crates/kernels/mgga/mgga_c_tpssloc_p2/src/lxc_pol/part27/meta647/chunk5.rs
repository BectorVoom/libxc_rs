//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2235/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2235<F: Float>(t1011: F, t4649: F, t10474: F, t381: F, t82514: F, t1615: F, t3032: F, t25483: F, t23384: F, t25456: F, t1049: F, t11065: F, t13980: F, t13985: F, t14590: F, t23346: F, t23601: F, t23602: F, t25459: F, t25484: F, t25485: F, t25486: F, t25487: F, t25516: F, t25714: F, t2780: F, t3127: F, t3132: F, t4594: F, t6687: F, t6784: F, t7619: F, t82513: F, t82534: F, t82694: F) -> (F, F, F) {
    let t89194 = t4649 * t1011;
    let t89204 = t82514 * t10474 * t381;
    let t89205 = t1615 * t3032;
    let t89210 = t82514 * t25483;
    let t89224 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25456;
    let t89225 = -F::cast_from(0.87729816898572076613e-1_f64) * t82534 * t25487 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6784 * t25516 * t2780 + F::cast_from(0.3289868133696452873e-1_f64) * t23601 * t23602 * t3127 * t1049 * t25486 + F::cast_from(0.3289868133696452873e-1_f64) * t23601 * t25484 * t89194 * t4594 + F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t25484 * t25485 * t13980 + F::cast_from(0.49348022005446793095e-1_f64) * t82513 * t89204 * t89205 * t13985 - F::cast_from(0.49348022005446793095e-1_f64) * t82513 * t89210 * t89205 * t3132 - F::new(6.0) * t11065 * t7619 * t14590 - F::cast_from(0.14621636149762012769e-1_f64) * t82694 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25714 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25459 - t89224;
    (t89194, t89205, t89225)
}
