//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2219/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2219<F: Float>(t14192: F, t6717: F, t13965: F, t6755: F, t25577: F, t3103: F, t25650: F, t3030: F, t82890: F, t1618: F, t23422: F, t23433: F, t23489: F, t23544: F, t25652: F, t25654: F, t25655: F, t25679: F, t3123: F, t3128: F, t4585: F, t4609: F, t4649: F, t4652: F, t7583: F, t82981: F, t83068: F, t83127: F) -> (F, F) {
    let t88636 = t6717 * t14192 / F::new(432.0);
    let t88645 = t6755 * t13965;
    let t88648 = t25577 * t3103 / F::new(1152.0);
    let t88655 = t25650 * t82890 * t3030;
    let t88662 = -t23422 * t4609 / F::new(54.0) + t88636 - t23544 * t4585 / F::new(576.0) + F::cast_from(0.10093189023535097714e-3_f64) * t82981 * t7583 + F::cast_from(0.20186378047070195428e-3_f64) * t23489 * t25679 + t25577 * t3123 / F::new(1536.0) - t88645 / F::new(6912.0) + t88648 + t83068 * t1618 / F::new(1536.0) + t23433 * t4652 / F::new(768.0) + F::cast_from(0.72670960969452703541e-2_f64) * t83127 + F::cast_from(0.40372756094140390856e-3_f64) * t88655 * t25655 + F::cast_from(0.40372756094140390856e-3_f64) * t25652 * t3128 * t4649 * t25654;
    (t88655, t88662)
}
