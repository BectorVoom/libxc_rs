//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1242/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1242<F: Float>(t22892: F, t22893: F, t32740: F, t552: F, t7722: F, t1307: F, t6637: F, t6888: F, t1992: F, t26404: F, t6976: F, t22897: F, t26453: F) -> (F, F, F, F) {
    let t120490 = t22892 * t22893 * t32740;
    let t120491 = F::cast_from(0.16449340668482264365e-1_f64) * t120490;
    let t120492 = t552 * t7722;
    let t120496 = F::cast_from(0.3289868133696452873e-1_f64) * t6888 * t6637 * t120492 * t1307;
    let t120502 = F::cast_from(0.16449340668482264365e-1_f64) * t1992 * t6976 * t26404;
    let t120505 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t22897 * t26453;
    (t120491, t120496, t120502, t120505)
}
