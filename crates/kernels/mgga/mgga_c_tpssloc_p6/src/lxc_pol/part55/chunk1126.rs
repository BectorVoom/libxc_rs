//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1126/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1126<F: Float>(t265: F, t504: F, t34321: F, t34352: F, t1256: F, t1763: F, t193: F, t32555: F, t32561: F, t33043: F, t336: F, t4700: F, t7398: F, t8090: F) -> (F, F) {
    let t505 = t265 < t504;
    let t34353 = t34321 + t34352;
    let t34366 = piecewise3::<F>(t505, t1256 * t193 * t336 * t34353 - t1763 * t32555 * t4700 + F::new(2.0) * t1763 * t32561 * t4700 - F::new(2.0) * t4700 * t7398 * t8090, t33043);
    (t34353, t34366)
}
