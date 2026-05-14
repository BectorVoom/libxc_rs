//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1202/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1202<F: Float>(t20173: F, t33193: F, t3941: F, t4072: F, t8326: F, t16524: F, t31285: F, t16521: F, t12524: F, t576: F, t1395: F, t1458: F, t33662: F, t26135: F, t7230: F, t7015: F, t94170: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t120800 = 27.0 * t20173 * t33193;
    let t120803 = 27.0 * t3941 * t8326 * t4072;
    let t120807 = 27.0 * t16524 * t31285;
    let t120809 = 0.135e2 * t16521 * t8326;
    let t120818 = 27.0 * t12524 * t33193;
    let t120833 = t576 * t4072;
    let t120849 = t1395 * t1458;
    let t120857 = t576 * t33662;
    let t120865 = 0.135e2 * t7230 * t26135;
    let t120867 = 27.0 * t94170 * t7015;
    (t120800, t120803, t120807, t120809, t120818, t120833, t120849, t120857, t120865, t120867)
}
