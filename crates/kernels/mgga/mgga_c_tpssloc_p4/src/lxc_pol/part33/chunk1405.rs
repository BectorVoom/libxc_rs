//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1405/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1405<F: Float>(t20356: F, t6889: F, t6890: F, t80732: F, t1843: F, t20029: F, t26366: F, t28187: F, t5321: F, t568: F, t6361: F, t6440: F, t7722: F, t7750: F, t81399: F, t91531: F, t91548: F, t97732: F, t97750: F, t97756: F) -> F {
    let t107484 = t80732 * t6889 * t6890 * t20356;
    let t107486 = -F::new(6.0) * t20029 * t7750 + F::new(3.0) * t6361 * t7722 * t568 + F::cast_from(0.49348022005446793095e-1_f64) * t97732 - F::cast_from(0.78134368175290755733e-1_f64) * t91531 - F::new(3.0) * t5321 * t28187 - F::cast_from(0.57572692339687925277e-1_f64) * t97750 + F::cast_from(0.49348022005446793095e-1_f64) * t91548 - F::new(6.0) * t97756 * t1843 - t81399 + F::new(6.0) * t26366 * t6440 - F::cast_from(0.19739208802178717238e0_f64) * t107484;
    t107486
}
