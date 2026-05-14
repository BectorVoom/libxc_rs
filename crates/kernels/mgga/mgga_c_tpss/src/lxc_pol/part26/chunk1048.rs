//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1048/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1048<F: Float>(t14538: F, t854: F, t847: F, t4847: F, t8684: F, t849: F, t3773: F, t3781: F, t2487: F, t4854: F, t8678: F, t3789: F, t2504: F, t11002: F, t11050: F, t11051: F, t11071: F, t8665: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14539 = t854 * t14538;
    let t14541 = t847 * t14538;
    let t14550 = t8684 * t4847;
    let t14551 = t14550 * t849;
    let t14553 = t3773 * t3781;
    let t14555 = t2487 * t4854;
    let t14556 = t14555 * t849;
    let t14558 = t8678 * t4847;
    let t14559 = t14558 * t849;
    let t14561 = t3789 * t3781;
    let t14563 = t2504 * t4854;
    let t14564 = t14563 * t849;
    let t14568 = -t8665 + 0.142419375e1 * t14551 - 0.1898925e1 * t14553 - 0.9494625e0 * t14556 - 0.76790625e-1 * t14559 + 0.3071625e0 * t14561 + 0.15358125e0 * t14564 - t11050 + 0.36514074074074074073e-1 * t11051 + 0.13287407407407407407e0 * t11002 - t11071;
    (t14539, t14541, t14551, t14553, t14556, t14559, t14561, t14564, t14568)
}
