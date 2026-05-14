//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 997/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk997<F: Float>(t1842: F, t3886: F, t1385: F, t22635: F, t1992: F, t6883: F, t7697: F, t1375: F, t16460: F, t2016: F, t26224: F, t26226: F, t26229: F, t26329: F, t26335: F, t26340: F, t26345: F, t26348: F, t26352: F, t3882: F, t5321: F, t568: F, t6963: F, t7729: F) -> (F, F) {
    let t26354 = t3886 * t1842;
    let t26355 = t26354 * t1385;
    let t26356 = t22635 * t26355;
    let t26357 = t1992 * t26356;
    let t26361 = t6883 * t7697;
    let t26364 = -6.0 * t26224 * t26226 + t26229 * t568 + t26329 * t568 + 0.49348022005446793095e-1 * t26335 + 0.16449340668482264365e-1 * t26340 + 2.0 * t3882 * t7729 + 0.41123351671205660912e-2 * t26345 + 2.0 * t1375 * t26348 - 0.82246703342411321825e-2 * t26352 + 0.16449340668482264365e-1 * t26357 + 2.0 * t5321 * t6963 - 0.19190897446562641759e-1 * t26361 - t16460 * t2016;
    (t26355, t26364)
}
