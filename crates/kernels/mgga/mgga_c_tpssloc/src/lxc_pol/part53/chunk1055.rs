//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1055/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1055<F: Float>(t2098: F, t7961: F, t1851: F, t8822: F, t34102: F, t576: F, t112: F, t34076: F, t117397: F, t120800: F, t120803: F, t120807: F, t120809: F, t120833: F, t120849: F, t16524: F, t2039: F, t27170: F, t27254: F, t31284: F, t32308: F, t33185: F, t33195: F, t5376: F, t671: F, t7056: F, t7230: F, t75795: F, t8508: F, t8717: F, t94127: F) -> (F, F, F, F) {
    let t124603 = t2098 * t7961;
    let t124609 = t1851 * t8822;
    let t124612 = t576 * t34102;
    let t124630 = t34076 * t112;
    let t124635 = t31284 + t8508 + F::new(27.0) * t120849 * t8717 + F::new(27.0) * t120833 * t8717 + F::new(27.0) * t75795 * t8717 + F::new(27.0) * t117397 * t5376 + F::new(27.0) * t27254 * t7056 + t33195 + F::new(54.0) * t16524 * t32308 + t120800 + t120803 + F::new(27.0) * t94127 * t2039 + F::new(27.0) * t7230 * t27170 + F::new(0.135e2) * t124630 * t671 + t120807 + F::new(54.0) * t33185 * t32308 + t120809;
    (t124603, t124609, t124612, t124635)
}
