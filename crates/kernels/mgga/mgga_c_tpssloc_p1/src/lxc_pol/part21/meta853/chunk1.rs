//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3083/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3083<F: Float>(t1113: F, t136: F, t63406: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63323: F) -> (F, F) {
    let t63939 = t136 * t1113 * t63406;
    let t63953 = -F::new(4.0) / F::new(9.0) * t63291 + F::new(4.0) / F::new(3.0) * t63296 + F::new(2.0) / F::new(3.0) * t63300 + F::new(2.0) * t63304 + F::new(4.0) / F::new(27.0) * t63306 - F::new(20.0) / F::new(81.0) * t63308 - F::new(4.0) / F::new(9.0) * t63313 - F::new(2.0) / F::new(9.0) * t63317 + F::new(16.0) / F::new(27.0) * t50826 - F::new(2.0) / F::new(9.0) * t50828 - F::new(56.0) / F::new(81.0) * t50834 + F::new(40.0) / F::new(27.0) * t63323;
    (t63939, t63953)
}
