//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1287/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1287<F: Float>(t22724: F, t22927: F, t22642: F, t22643: F, t6907: F, t1307: F, t22633: F, t22635: F, t3886: F, t3888: F, t12437: F, t1375: F, t1378: F, t1385: F, t2015: F, t2016: F, t22656: F, t22904: F, t22913: F, t3882: F, t3887: F, t3912: F, t39913: F, t39919: F, t80744: F, t81063: F, t81117: F, t81183: F, t81250: F) -> F {
    let t81264 = t22724 * t22927;
    let t81267 = t22642 * t22643 * t6907;
    let t81272 = t22633 * t22635 * t3886 * t3888 * t1307;
    let t81278 = -t80744 - F::new(3.0) * t39913 * t2016 - t1375 * t1378 * (t81063 + t81117 + t81183 + t81250) - F::new(3.0) * t22656 * t3912 - t39919 * t2016 + F::new(2.0) * t1375 * t3887 * t2015 * t12437 + F::new(6.0) * t3882 * t22913 + F::cast_from(0.78134368175290755733e-1_f64) * t81264 + F::cast_from(0.24674011002723396547e-1_f64) * t81267 - F::cast_from(0.9869604401089358619e-1_f64) * t81272 + F::new(6.0) * t1375 * t3887 * t22904 * t1385;
    t81278
}
