//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 592/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk592<F: Float>(t28: F, t265: F, t504: F, t1081: F, t1877: F, t1915: F, t2522: F, t6666: F, t6670: F, t6841: F, t6848: F, t6834: F, t1972: F, t52: F, t607: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t6855 = F::new(3.0) / F::new(2.0) * t2522 * t1915 * t6841 + t1877 * t6666 * t28 / F::new(2.0) - t1877 * t6670 * t6848 / F::new(2.0) + t1877 * t1915 * t1081 / F::new(2.0);
    let t6856 = piecewise3::<f64>(t505, F::new(0.0), t6834);
    let t6861 = piecewise3::<f64>(t401, t6855, -t1972 * t607 / F::new(2.0) + t6856 * t52 / F::new(2.0));
    (t6856, t6861)
}
