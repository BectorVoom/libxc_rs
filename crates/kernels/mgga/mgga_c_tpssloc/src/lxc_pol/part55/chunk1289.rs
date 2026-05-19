//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1289/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1289<F: Float>(t28: F, t265: F, t504: F, t119677: F, t125789: F, t119784: F, t1409: F, t32566: F, t34366: F, t3966: F, t52: F, t607: F, t8909: F, t113: F, t120002: F, t120008: F, t120019: F, t123044: F, t123119: F, t123120: F, t123122: F, t123124: F, t123126: F, t123129: F, t123138: F, t123140: F, t123142: F, t1393: F, t24932: F, t27879: F, t27888: F, t34381: F, t7266: F, t7408: F, t7983: F, t7989: F, t8329: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t125790 = piecewise3::<F>(t505, t125789, t119677);
    let t125797 = piecewise3::<F>(t401, t119784, t125790 * t52 / F::new(2.0) - t32566 * t1409 / F::new(2.0) - t34366 * t607 / F::new(2.0) - t8909 * t3966 / F::new(2.0));
    let t125802 = -F::new(2.0) * t123119 + t34381 * t1393 + t120002 - t8329 - F::new(4.0) * t123120 - F::new(4.0) * t123122 - F::new(4.0) * t123124 - F::new(4.0) * t123126 - F::new(4.0) * t123129 - t120008 - F::new(4.0) * t123138 - F::new(4.0) * t123140 - F::new(4.0) * t123142 - t120019 - F::new(4.0) * t24932 * t7989 - F::new(4.0) * t27888 * t7989 - F::new(4.0) * t7266 * t27879 - t113 * (t123044 + t125797) - F::new(2.0) * t7983 * t7408;
    t125802
}
