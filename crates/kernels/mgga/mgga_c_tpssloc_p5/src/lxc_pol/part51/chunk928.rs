//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 928/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk928<F: Float>(t22852: F, t22855: F, t2003: F, t3862: F, t1358: F, t6940: F, t1887: F, t22715: F, t534: F, t1995: F, t9223: F, t213: F) -> (F, F, F, F, F, F, F, F) {
    let t22856 = t22852 * t22855;
    let t22858 = t2003 * t3862;
    let t22859 = F::new(119.0) / F::new(6912.0) * t22858;
    let t22860 = t6940 * t1358;
    let t22861 = F::new(7.0) / F::new(1152.0) * t22860;
    let t22863 = t22715 * t534 * t1887;
    let t22864 = F::new(35.0) / F::new(432.0) * t22863;
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    (t22856, t22858, t22859, t22860, t22861, t22863, t22864, t22866)
}
