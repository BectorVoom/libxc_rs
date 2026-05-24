//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1061/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1061<F: Float>(t2504: F, t4854: F, t849: F, t11002: F, t11050: F, t11051: F, t11071: F, t14551: F, t14553: F, t14556: F, t14559: F, t14561: F, t8665: F) -> (F, F) {
    let t14563 = t2504 * t4854;
    let t14564 = t14563 * t849;
    let t14568 = -t8665 + F::cast_from(0.142419375e1_f64) * t14551 - F::new(0.1898925e1) * t14553 - F::new(0.9494625e0) * t14556 - F::new(0.76790625e-1) * t14559 + F::new(0.3071625e0) * t14561 + F::new(0.15358125e0) * t14564 - t11050 + F::cast_from(0.36514074074074074073e-1_f64) * t11051 + F::cast_from(0.13287407407407407407e0_f64) * t11002 - t11071;
    (t14564, t14568)
}
