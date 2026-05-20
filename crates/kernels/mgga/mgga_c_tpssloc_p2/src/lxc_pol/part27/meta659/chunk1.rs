//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2302/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2302<F: Float>(t1332: F, t26401: F, t90747: F, t90750: F, t90752: F, t90757: F, t90760: F, t90763: F, t90770: F, t90774: F, t90778: F, t90782: F, t90785: F, t90789: F, t90792: F, t90795: F, t90798: F, t90801: F) -> F {
    let t90803 = F::cast_from(0.16449340668482264365e-1_f64) * t90747 - t90750 - F::cast_from(0.82246703342411321825e-2_f64) * t90752 + F::cast_from(0.9869604401089358619e-1_f64) * t90757 + t90760 - F::cast_from(0.49348022005446793095e-1_f64) * t90763 + F::new(2.0) * t1332 * t26401 - F::cast_from(0.16449340668482264365e-1_f64) * t90770 + F::cast_from(0.3289868133696452873e-1_f64) * t90774 + F::cast_from(0.16449340668482264365e-1_f64) * t90778 + t90782 - F::cast_from(0.82246703342411321825e-2_f64) * t90785 - F::cast_from(0.49348022005446793096e-1_f64) * t90789 + t90792 + t90795 + t90798 - F::cast_from(0.16449340668482264365e-1_f64) * t90801;
    t90803
}
