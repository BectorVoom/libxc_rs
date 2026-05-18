//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1414/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1414<F: Float>(t28: F, t265: F, t504: F, t83607: F, t83654: F, t83543: F, t1972: F, t2250: F, t23821: F, t52: F, t607: F, t6856: F, t9258: F, t22561: F, t2314: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t83655 = t83607 + t83654;
    let t83656 = piecewise3::<f64>(t505, F::new(0.0), t83543);
    let t83666 = piecewise3::<f64>(t401, t83655, t83656 * t52 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t23821 * t607 - F::new(3.0) / F::new(2.0) * t6856 * t2250 - t1972 * t9258 / F::new(2.0));
    let t83672 = F::new(12.0) * t2314 * t22561;
    (t83666, t83672)
}
