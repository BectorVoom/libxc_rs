//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2136/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2136<F: Float>(t1992: F, t550: F, t57545: F, t6976: F, t90750: F, t90760: F, t90782: F, t90789: F, t90792: F, t90795: F, t90798: F, t90806: F, t90807: F, t93517: F, t96935: F, t96937: F, t96941: F, t96945: F, t96949: F, t96954: F) -> F {
    let t96958 = t1992 * t6976 * t57545 * t550;
    let t96960 = F::cast_from(0.3289868133696452873e-1_f64) * t96935 - F::cast_from(0.38381794893125283518e-1_f64) * t96937 - t90750 + t90760 - F::cast_from(0.82246703342411321825e-2_f64) * t96941 + t90782 - F::cast_from(0.49348022005446793095e-1_f64) * t90789 + t90792 + t90795 + t90798 + t90806 - F::cast_from(0.25587863262083522345e0_f64) * t90807 + F::cast_from(0.19190897446562641759e-1_f64) * t96945 - F::cast_from(0.82246703342411321825e-2_f64) * t96949 + F::cast_from(0.49348022005446793095e-1_f64) * t96954 - F::cast_from(0.16449340668482264365e-1_f64) * t96958 - t93517;
    t96960
}
