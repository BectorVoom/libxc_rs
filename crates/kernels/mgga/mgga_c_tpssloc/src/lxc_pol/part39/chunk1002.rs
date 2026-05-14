//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1002/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1002<F: Float>(t13654: F, t913: F, t893: F, t2929: F, t4471: F, t4497: F, t959: F, t2904: F, t952: F, t3216: F, t4696: F, t13550: F, t13563: F, t10296: F, t10298: F, t10302: F, t13566: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F) -> (F, F, F, F, F, F, F) {
    let t13655 = t13654 * t913;
    let t13657 = 1.0 * t893 * t13655;
    let t13658 = t2929 * t4471;
    let t13659 = t13658 * t4497;
    let t13661 = 0.34631718211362927518e2 * t959 * t13659;
    let t13662 = t2904 * t4471;
    let t13663 = t13662 * t952;
    let t13665 = 0.23392894490538584828e1 * t959 * t13663;
    let t13666 = t4696 * t3216;
    let t13675 = 0.22076e0 * t13550;
    let t13679 = 0.13418888888888888889e0 * t13563;
    let t13692 = -0.40256666666666666667e0 * t13566 - 0.33547222222222222222e0 * t13569 + 0.12077e1 * t13572 - 0.40256666666666666666e0 * t13575 - 0.20128333333333333333e0 * t13578 - 0.181155e1 * t13581 + 0.12077e1 * t13584 + 0.60385e0 * t13587 - 0.18396666666666666667e0 * t10296 + 0.5519e-1 * t10302 + 0.18396666666666666667e-1 * t10298;
    (t13657, t13661, t13665, t13666, t13675, t13679, t13692)
}
