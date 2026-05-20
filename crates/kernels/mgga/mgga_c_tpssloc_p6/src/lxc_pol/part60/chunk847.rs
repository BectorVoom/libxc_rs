//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 847/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk847<F: Float>(t265: F, t504: F, t1256: F, t1763: F, t193: F, t24909: F, t27838: F, t28755: F, t29827: F, t336: F, t4700: F, t6270: F, t6274: F, t7398: F) -> F {
    let t505 = t265 < t504;
    let t29840 = piecewise3::<F>(t505, t1256 * t193 * t29827 * t336 - F::new(2.0) * t1763 * t27838 * t4700 + F::new(2.0) * t24909 * t4700 * t6274 - t4700 * t6270 * t7398, t28755);
    t29840
}
