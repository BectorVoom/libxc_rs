//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2328/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2328<F: Float>(t28: F, t265: F, t504: F, t100674: F, t100716: F, t100763: F, t100803: F, t100624: F, t1409: F, t16558: F, t1972: F, t25950: F, t28803: F, t3966: F, t52: F, t5398: F, t607: F, t6856: F, t7664: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t100805 = t100674 + t100716 + t100763 + t100803;
    let t100806 = piecewise3::<F>(t505, F::new(0.0), t100624);
    let t100818 = piecewise3::<F>(t401, t100805, t100806 * t52 / F::new(2.0) - t28803 * t607 / F::new(2.0) - t25950 * t1409 - t7664 * t3966 - t6856 * t5398 / F::new(2.0) - t1972 * t16558 / F::new(2.0));
    t100818
}
