//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2260/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2260<F: Float>(t28: F, t265: F, t504: F, t89880: F, t89920: F, t89957: F, t90001: F, t89823: F, t12606: F, t1409: F, t1972: F, t2250: F, t23821: F, t25950: F, t3966: F, t52: F, t607: F, t6856: F, t7664: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t90003 = t89880 + t89920 + t89957 + t90001;
    let t90004 = piecewise3::<F>(t505, F::new(0.0), t89823);
    let t90016 = piecewise3::<F>(t401, t90003, t90004 * t52 / F::new(2.0) - t25950 * t607 - t7664 * t2250 / F::new(2.0) - t23821 * t1409 / F::new(2.0) - t6856 * t3966 - t1972 * t12606 / F::new(2.0));
    t90016
}
