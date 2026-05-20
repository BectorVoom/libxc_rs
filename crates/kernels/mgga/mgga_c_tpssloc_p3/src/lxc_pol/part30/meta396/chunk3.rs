//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1513/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1513<F: Float>(t1569: F, t4433: F, t5762: F, t931: F, t5759: F, t2888: F, t5758: F, t4437: F, t10813: F, t5742: F, t10771: F, t10811: F, t14271: F, t14276: F, t17519: F, t17523: F, t17526: F, t17530: F, t17535: F, t2861: F, t2886: F, t4416: F, t4438: F) -> F {
    let t17538 = t1569 * t4433;
    let t17541 = t5762 * t931;
    let t17544 = t5759 * t931;
    let t17547 = t5758 * t2888;
    let t17548 = t17547 * t931;
    let t17551 = t4437 * t4433;
    let t17554 = t5742 * t10813;
    let t17555 = t17554 * t931;
    let t17558 = t17519 - t17523 - t17526 - t17530 - F::new(4.0) * t14276 * t4416 + F::cast_from(0.64327917994770140268e2_f64) * t14271 * t4438 + F::new(6.0) * t2886 * t17535 - F::new(4.0) * t2861 * t17538 - F::cast_from(0.19298375398431042081e3_f64) * t10771 * t17541 - F::new(2.0) * t2861 * t17544 + F::cast_from(0.32163958997385070134e2_f64) * t2886 * t17548 + F::cast_from(0.64327917994770140268e2_f64) * t2886 * t17551 + F::cast_from(0.2069040516770936012e4_f64) * t10811 * t17555;
    t17558
}
