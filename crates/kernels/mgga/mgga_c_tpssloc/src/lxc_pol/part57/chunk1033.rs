//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1033/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1033<F: Float>(t1985: F, t29360: F, t6889: F, t6906: F, t122142: F, t1842: F, t1992: F, t22635: F, t115551: F, t115567: F, t122172: F, t122281: F, t122295: F, t127346: F, t127349: F, t127350: F, t1807: F, t1843: F, t20044: F, t26366: F, t28111: F, t29311: F, t33266: F, t33294: F, t5321: F, t568: F, t6958: F, t7194: F, t7925: F, t7937: F, t8627: F) -> F {
    let t128768 = t1985 * t6889 * t6906 * t29360;
    let t128781 = t1992 * t22635 * t122142 * t1842;
    let t128789 = t115551 - F::new(2.0) * t26366 * t7937 - F::new(2.0) * t5321 * t33294 - F::new(0.82246703342411321825e-2) * t128768 - t127346 + F::new(4.0) * t26366 * t7925 + F::new(2.0) * t7194 * t28111 + F::new(4.0) * t6958 * t29311 - F::new(0.16449340668482264365e-1) * t122281 + F::new(2.0) * t20044 * t8627 + F::new(0.3289868133696452873e-1) * t128781 - t127349 - t127350 + t115567 - F::new(2.0) * t122172 * t1843 + F::new(2.0) * t1807 * t33266 * t568 + F::new(0.38381794893125283518e-1) * t122295;
    t128789
}
