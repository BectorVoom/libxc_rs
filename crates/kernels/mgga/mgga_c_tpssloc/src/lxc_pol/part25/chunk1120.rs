//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1120/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1120<F: Float>(t1385: F, t1992: F, t22635: F, t3886: F, t3911: F, t22649: F, t6883: F, t1372: F, t212: F, t22642: F, t6890: F, t1985: F, t22666: F, t22934: F) -> (F, F, F, F) {
    let t81305 = t1992 * t22635 * t3886 * t1385 * t3911;
    let t81307 = t6883 * t22649;
    let t81311 = t22642 * t212 * t1372 * t6890;
    let t81315 = t1985 * t22666 * t22934;
    (t81305, t81307, t81311, t81315)
}
