//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1469/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1469<F: Float>(t75795: F, t8657: F, t100993: F, t7769: F, t24465: F, t26542: F, t26545: F, t112: F, t33627: F, t16524: F, t31817: F, t115984: F, t115996: F, t120807: F, t120809: F, t120818: F, t1458: F, t23880: F, t27170: F, t27281: F, t5376: F, t671: F, t7010: F) -> F {
    let t122800 = F::cast_from(27.0_f64) * t75795 * t8657;
    let t122804 = F::cast_from(27.0_f64) * t100993 * t7769;
    let t122806 = F::cast_from(27.0_f64) * t24465 * t26542;
    let t122808 = F::cast_from(27.0_f64) * t24465 * t26545;
    let t122811 = t33627 * t112;
    let t122817 = F::cast_from(27.0_f64) * t16524 * t31817;
    let t122820 = t122800 + F::cast_from(0.135e2_f64) * t7010 * t27170 + t120807 + t122804 + t122806 + t122808 + F::cast_from(27.0_f64) * t115984 * t5376 + F::cast_from(0.135e2_f64) * t122811 * t671 + t120809 + F::cast_from(27.0_f64) * t23880 * t27281 + t122817 + t120818 + F::cast_from(0.135e2_f64) * t115996 * t1458;
    t122820
}
